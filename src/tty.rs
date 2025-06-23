use std::io;
use std::os::fd::AsFd;
#[cfg(unix)]
use nix::{
    libc,
    sys::termios::tcgetattr,
    unistd::isatty
};

/// TTY security status
pub struct TtySecurity {
    pub is_secure: bool,
    pub warnings: Vec<String>,
}

impl TtySecurity {
    /// Check if the TTY environment is secure
    pub fn check() -> io::Result<Self> {
        let mut warnings = Vec::new();
        let mut secure = true;

        #[cfg(unix)] {
            // 1. Basic TTY checks
            let stdin = std::io::stdin();
            if !isatty(stdin.as_fd())? {
                warnings.push("Stdin is not a real TTY".to_string());
                secure = false;
            }

            // 2. Check for known logging programs
            for (var, reason) in [
                ("SCRIPT", "Running under 'script' command"),
                ("TERM_PROGRAM", "Terminal emulator may log output"),
                ("TMUX", "Tmux may maintain history"),
            ] {
                if std::env::var(var).is_ok() {
                    warnings.push(format!("{} detected: {}", var, reason));
                    secure = false;
                }
            }

            // 3. Verify terminal attributes
            if let Err(e) = tcgetattr(&std::fs::File::open("/dev/tty")?) {
                warnings.push(format!("Failed to get terminal attributes: {}", e));
                secure = false;
            }
        }

        #[cfg(not(unix))] {
            warnings.push("TTY security checks not supported on this platform".to_string());
            secure = false;
        }

        Ok(Self { is_secure: secure, warnings })
    }

    /// Apply TTY hardening measures
    pub fn harden(&self) -> io::Result<()> {
        if !self.is_secure {
            for warning in &self.warnings {
                eprintln!("(incogt) TTY warning: {}", warning);
            }
        }
        Ok(())
    }
}