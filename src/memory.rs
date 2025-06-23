use std::io::{self, Error, ErrorKind};

#[cfg(unix)]
use nix::{
    errno::Errno,
    sys::mman::{mlockall, MlockAllFlags},
    sys::resource::{setrlimit, Resource}
};

/// Memory protection configuration
pub struct MemoryProtection {
    pub memory_locked: bool,
    pub core_dumps_disabled: bool,
}

impl MemoryProtection {
    /// Initialize memory protection for the current process
    pub fn initialize() -> io::Result<Self> {
        let mut protection = MemoryProtection {
            memory_locked: false,
            core_dumps_disabled: false,
        };

        // Step 1: Disable core dumps first
        protection.disable_core_dumps()?;

        // Step 2: Lock memory pages
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
            Err(Errno::EPERM) => {
                eprintln!("(incogt) Warning: Cannot lock memory - insufficient permissions");
                eprintln!("(incogt) Consider running with CAP_IPC_LOCK capability or as root");
                Ok(())
            }
            Err(Errno::ENOMEM) => {
                eprintln!("(incogt) Warning: Cannot lock memory - insufficient memory");
                Ok(())
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Failed to lock memory: {}", e),
            )),
        }
    }

    #[cfg(not(unix))]
    fn lock_memory(&mut self) -> io::Result<()> {
        eprintln!("(incogt) Warning: Memory locking not supported on this platform");
        Ok(())
    }

    /// Disable core dumps
    #[cfg(unix)]
    fn disable_core_dumps(&mut self) -> io::Result<()> {
        match setrlimit(Resource::RLIMIT_CORE, 0, 0) {
            Ok(()) => {
                self.core_dumps_disabled = true;
                Ok(())
            }
            Err(e) => {
                eprintln!("(incogt) Warning: Cannot disable core dumps: {}", e);
                Ok(())
            }
        }
    }

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
    fn drop(&mut self) {
        if self.memory_locked {
            println!("(incogt) Memory protection cleanup - memory remains locked until exit");
        }
    }
}