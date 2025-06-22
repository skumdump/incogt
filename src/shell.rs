/// Generates the shell command string for an incognito session based on the shell name.
/// Returns `Ok(String)` with the command or `Err` for unsupported shells.
pub fn get_shell_cmd(shell_name: &str) -> Result<String, std::io::Error> {
    match shell_name {
        "bash" => Ok(r#"
            export HISTFILE=/dev/null
            export HISTSIZE=1000
            export HISTFILESIZE=0
            export PS1="(incogt) [\u@\h \W]\$ "
            exec bash --norc --noprofile -i
        "#.to_string()),
        "zsh" => Ok(r#"
            export HISTFILE=/dev/null
            export HISTSIZE=1000
            export SAVEHIST=0
            export PS1="(incogt) [%n@%m %~]$ "
            exec zsh --no-rcs -i
        "#.to_string()),
        "fish" => Ok(r#"
            set fish_history none
            set fish_prompt_pwd_dir_length 1
            function fish_prompt; echo "(incogt) [$USER@$HOSTNAME (prompt_pwd)]$ "; end
            exec fish --no-config -i
        "#.to_string()),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Unsupported shell: {}", shell_name),
        )),
    }
}