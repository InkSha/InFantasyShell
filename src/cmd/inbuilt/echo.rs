use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("echo");

    cmd.with_casesensitive(false).with_handle(|args, state| {
        if args.is_empty() {
            return command::CommandOutput::DISPLAY(String::new());
        }

        if let Some(redirect_index) = args.iter().position(|arg| arg == ">") {
            if redirect_index + 1 >= args.len() {
                return command::CommandOutput::ERROR(
                    "echo redirection requires a target path".to_string(),
                );
            }

            let content = args[..redirect_index].join(" ");
            let path = &args[redirect_index + 1];

            return match state.vfs.write_file(
                state.cwd,
                path.as_str(),
                state.actor.as_str(),
                content,
            ) {
                Ok(()) => command::CommandOutput::NONE,
                Err(error) => command::CommandOutput::ERROR(error.to_string()),
            };
        }

        command::CommandOutput::DISPLAY(args.join(" "))
    });

    cmd
}

#[cfg(test)]
mod tests {
    use crate::cmd::command::CommandOutput;
    use crate::cmd::runtime::ShellState;

    use super::register_command;

    #[test]
    fn echo_without_redirection_displays_text() {
        let command = register_command();
        let mut state = ShellState::new("player").expect("shell state should initialize");
        let args = vec!["hello".to_string(), "world".to_string()];

        let output = command.execute(&args, &mut state);

        match output {
            CommandOutput::DISPLAY(content) => assert_eq!(content, "hello world"),
            _ => panic!("echo should display plain text"),
        }
    }

    #[test]
    fn echo_redirection_writes_into_vfs() {
        let command = register_command();
        let mut state = ShellState::new("player").expect("shell state should initialize");
        let args = vec![
            "quest".to_string(),
            "log".to_string(),
            ">".to_string(),
            "journal.txt".to_string(),
        ];

        let output = command.execute(&args, &mut state);

        assert!(matches!(output, CommandOutput::NONE));
        assert_eq!(
            state
                .vfs
                .read_file(state.cwd, "journal.txt", state.actor.as_str())
                .expect("journal should be written"),
            "quest log"
        );
    }
}
