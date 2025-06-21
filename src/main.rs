use std::env;
use std::io;
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    println!("(incogt) Starting incognito shell session with shell: {}", shell);

    // Command to set PS1 and start interactive shell with normal configs
    let shell_cmd = r#"
export HISTFILE=/dev/null
export HISTSIZE=1000
export HISTFILESIZE=0
PS1="(incogt) [\\u@\\h \\W]\\$ "
exec bash -i
"#;

    let mut child = Command::new(shell)
        .arg("-c")
        .arg(shell_cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;

    println!("(incogt) Incognito session ended.");

    std::process::exit(status.code().unwrap_or(0));
}
