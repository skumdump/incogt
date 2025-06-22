use std::io::{self, Error, ErrorKind};

#[cfg(unix)]
use nix::sys::mman::{mlockall, MlockAllFlags};

#[cfg(unix)]
use nix::sys::resource::{setrlimit, Resource};

/// Memory protection configuration
pub struct MemoryProtection {
    /// Whether memory locking was successfully enabled
    pub memory_locked: bool,
    /// Whether core dumps were disabled
    pub core_dumps_disabled: bool,
}

impl MemoryProtection {
    /// Initialize memory protection for the current process
    /// This should be called early in main() before spawning the shell
    pub fn initialize() -> io::Result<Self> {
        let mut protection = MemoryProtection {
            memory_locked: false,
            core_dumps_disabled: false,
        };

        // Step 1: Disable core dumps first (prevents memory dumps on crash)
        protection.disable_core_dumps()?;

        // Step 2: Lock all current and future memory pages
        protection.lock_memory()?;

        println!("(incogt) Memory protection initialized");
        if protection.memory_locked {
            println!("(incogt) Memory locked - history cannot be swapped to disk");
        }
        if protection.core_dumps_disabled {
            println!("(incogt) Core dumps disabled - prevents crash memory dumps");
        }

        Ok(protection)
    }

    /// Lock all memory pages to prevent swapping
    #[cfg(unix)]
    fn lock_memory(&mut self) -> io::Result<()> {
        match mlockall(MlockAllFlags::MCL_CURRENT | MlockAllFlags::MCL_FUTURE) {
            Ok(()) => {
                self.memory_locked = true;
                Ok(())
            }
            Err(nix::errno::Errno::EPERM) => {
                eprintln!("(incogt) Warning: Cannot lock memory - insufficient permissions");
                eprintln!("(incogt) Consider running with CAP_IPC_LOCK capability or as root");
                eprintln!("(incogt) History may be swapped to disk!");
                Ok(()) // Don't fail, just warn
            }
            Err(nix::errno::Errno::ENOMEM) => {
                eprintln!("(incogt) Warning: Cannot lock memory - insufficient memory");
                eprintln!("(incogt) History may be swapped to disk!");
                Ok(()) // Don't fail, just warn
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Failed to lock memory: {}", e),
            )),
        }
    }

    /// Lock memory (no-op on non-Unix systems)
    #[cfg(not(unix))]
    fn lock_memory(&mut self) -> io::Result<()> {
        eprintln!("(incogt) Warning: Memory locking not supported on this platform");
        eprintln!("(incogt) History may be swapped to disk!");
        Ok(())
    }

    /// Disable core dumps to prevent memory dumps on crash
    #[cfg(unix)]
    fn disable_core_dumps(&mut self) -> io::Result<()> {
        use nix::sys::resource::{getrlimit, setrlimit, Resource};

        // Set core dump size limit to 0
        match setrlimit(Resource::RLIMIT_CORE, 0, 0) {
            Ok(()) => {
                self.core_dumps_disabled = true;
                Ok(())
            }
            Err(e) => {
                eprintln!("(incogt) Warning: Cannot disable core dumps: {}", e);
                Ok(()) // Don't fail, just warn
            }
        }
    }

    /// Disable core dumps (no-op on non-Unix systems)
    #[cfg(not(unix))]
    fn disable_core_dumps(&mut self) -> io::Result<()> {
        eprintln!("(incogt) Warning: Core dump disabling not supported on this platform");
        Ok(())
    }

    /// Check if memory protection is fully enabled
    pub fn is_fully_protected(&self) -> bool {
        self.memory_locked && self.core_dumps_disabled
    }

    /// Get protection status summary
    pub fn status_summary(&self) -> String {
        let memory_status = if self.memory_locked { "LOCKED" } else { "UNLOCKED" };
        let core_status = if self.core_dumps_disabled { "DISABLED" } else { "ENABLED" };
        format!("Memory: {}, Core dumps: {}", memory_status, core_status)
    }
}

impl Drop for MemoryProtection {
    /// Cleanup when the protection object is dropped
    /// Note: We don't unlock memory here as that could allow swapping
    /// Memory will be automatically unlocked when the process exits
    fn drop(&mut self) {
        if self.memory_locked {
            println!("(incogt) Memory protection cleanup - memory remains locked until exit");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_protection_initialization() {
        // This test may fail without proper permissions, but shouldn't panic
        let result = MemoryProtection::initialize();
        assert!(result.is_ok());
    }

    #[test]
    fn test_status_summary() {
        let protection = MemoryProtection {
            memory_locked: true,
            core_dumps_disabled: true,
        };
        assert!(protection.status_summary().contains("LOCKED"));
        assert!(protection.status_summary().contains("DISABLED"));
    }
}