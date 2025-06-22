use std::env;
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use std::process::{Command, Stdio};

// Import the shell module
mod shell;

/// Launches an "incognito" interactive shell session with disabled history and no user configuration files loaded.
/// Uses the `SHELL` environment variable or defaults to `/bin/bash`.
fn main() -> io::Result<()> {
    // Retrieve the user’s preferred shell
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    // Extract the shell name (e.g., "bash" from "/bin/bash")
    let shell_name = Path::new(&shell)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("bash");

    println!("(incogt) Starting incognito shell session with shell: {}", shell);

    // Get the shell-specific command
    let shell_cmd = shell::get_shell_cmd(shell_name)?;

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
    let status = shell_process.wait()?;

    println!("(incogt) Incognito session ended.");

    // Exit with the shell’s status code
    std::process::exit(status.code().unwrap_or(1));
}