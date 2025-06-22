use std::io::{Error, ErrorKind};

/// Generates the shell command string for an incognito session based on the shell name.
/// Returns `Ok(String)` with the command or `Err` for unsupported shells.
pub fn get_shell_cmd(shell_name: &str) -> Result<String, Error> {
    match shell_name {
        "bash" => Ok(format!(
            r#"
            export HISTFILE=/dev/null
            export HISTSIZE=1000
            export HISTFILESIZE=0
            export PS1="{} "
            unset HISTFILE
            exec bash --norc --noprofile -i
            "#,
            get_prompt()
        )),
        "zsh" => Ok(format!(
            r#"
            export HISTFILE=/dev/null
            export HISTSIZE=1000
            export SAVEHIST=0
            export PS1="{} "
            unset HISTFILE
            exec zsh --no-rcs -i
            "#,
            get_prompt()
        )),
        "fish" => Ok(format!(
            r#"
            set -e fish_history
            set -U fish_history ''
            function fish_prompt
                echo '{} '
            end
            exec fish --no-config -i
            "#,
            get_prompt()
        )),
        "sh" => Ok(format!(
            r#"
            export HISTFILE=/dev/null
            export HISTSIZE=0
            export PS1="{} "
            unset HISTFILE
            exec sh -i
            "#,
            get_prompt()
        )),
        "dash" => Ok(format!(
            r#"
            export HISTFILE=/dev/null
            export HISTSIZE=0
            export PS1="{} "
            unset HISTFILE
            exec dash -i
            "#,
            get_prompt()
        )),
        _ => Err(Error::new(
            ErrorKind::Unsupported,
            format!(
                "Unsupported shell: '{}'. Supported shells: bash, zsh, fish, sh, dash",
                shell_name
            ),
        )),
    }
}

/// Returns a consistent prompt string for all shells
fn get_prompt() -> &'static str {
    "(incogt) \\u@\\h:\\w\\$"
}

/// Returns the list of supported shells
pub fn supported_shells() -> &'static [&'static str] {
    &["bash", "zsh", "fish", "sh", "dash"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_shells() {
        for shell in supported_shells() {
            assert!(get_shell_cmd(shell).is_ok(), "Shell {} should be supported", shell);
        }
    }

    #[test]
    fn test_unsupported_shell() {
        assert!(get_shell_cmd("nonexistent").is_err());
    }

    #[test]
    fn test_shell_commands_contain_prompt() {
        for shell in supported_shells() {
            let cmd = get_shell_cmd(shell).unwrap();
            assert!(cmd.contains("incogt"), "Command for {} should contain 'incogt'", shell);
        }
    }
}