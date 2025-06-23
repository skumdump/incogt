use std::env;
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use std::process::{Command, Stdio};

mod memory;
mod shell;
mod tty;

fn main() -> io::Result<()> {
    // 1. Initialize memory protection FIRST
    let _memory_protection = memory::MemoryProtection::initialize()?;

    // 2. Check TTY security
    let tty_sec = tty::TtySecurity::check()?;
    tty_sec.harden()?;

    if !tty_sec.is_secure {
        eprintln!("(incogt) WARNING: Terminal environment may be logged");
        if !confirm_continue("Continue despite insecure terminal?")? {
            return Ok(());
        }
    }

    // 3. Shell detection and validation
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    if !Path::new(&shell).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Shell '{}' not found", shell)
        ));
    }

    let shell_name = Path::new(&shell)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("bash");

    // 4. Start secured shell session
    start_incognito_session(shell_name, &shell)?;

    Ok(())
}

fn start_incognito_session(shell_name: &str, shell_path: &str) -> io::Result<()> {
    println!("(incogt) Starting incognito session with: {}", shell_path);
    println!("(incogt) Use 'exit' or Ctrl+D to end the session");

    let shell_cmd = match shell::get_shell_cmd(shell_name) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Supported shells: {}", shell::supported_shells().join(", "));
            std::process::exit(1);
        }
    };

    // Safe because we control all inputs and validate them
    Command::new(shell_path)
        .arg("-c")
        .arg(&shell_cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}

fn confirm_continue(prompt: &str) -> io::Result<bool> {
    use std::io::{stdin, stdout, Write};

    print!("(incogt) {} [y/N]: ", prompt);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_shell_detection() {
        env::set_var("SHELL", "/bin/bash");
        let shell = env::var("SHELL").unwrap();
        assert!(Path::new(&shell).exists());
    }

    #[test]
    fn test_confirm_continue() {
        let result = confirm_continue("test");
        assert!(result.is_ok());
    }
}