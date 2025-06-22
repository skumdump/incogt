use std::env;
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use std::process::{Command, Stdio};

// Import the shell module
mod shell;

/// Launches an "incognito" interactive shell session with disabled history and no user configuration files loaded.
/// Uses the `SHELL` environment variable or defaults to `/bin/bash`.
fn main() -> io::Result<()> {
    // Retrieve the user's preferred shell
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    // Validate shell exists
    if !Path::new(&shell).exists() {
        eprintln!("Error: Shell '{}' not found", shell);
        std::process::exit(1);
    }

    // Extract the shell name (e.g., "bash" from "/bin/bash")
    let shell_name = Path::new(&shell)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("bash");

    println!("(incogt) Starting incognito shell session with: {}", shell);
    println!("(incogt) Use 'exit' or Ctrl+D to end the session");

    // Get the shell-specific command
    let shell_cmd = match shell::get_shell_cmd(shell_name) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Supported shells: {}", shell::supported_shells().join(", "));
            std::process::exit(1);
        }
    };

    // Spawn the shell process
    let mut shell_process = Command::new(&shell)
        .arg("-c")
        .arg(&shell_cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to spawn shell {}: {}", shell, e)))?;

    // Wait for the shell to exit
    let status = shell_process.wait().map_err(|e| {
        Error::new(ErrorKind::Other, format!("Error waiting for shell process: {}", e))
    })?;

    println!("(incogt) Incognito session ended.");
    std::process::exit(status.code().unwrap_or(0));
}