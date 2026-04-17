use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("clear");

    cmd.with_casesensitive(false)
        .with_handle(|_, _| command::CommandOutput::CLEAN);

    cmd
}

#[cfg(test)]
mod tests {
    use crate::cmd::{command::CommandOutput, runtime::ShellState};

    use super::register_command;

    #[test]
    fn clear_returns_clean_output() {
        let command = register_command();
        let mut state = ShellState::new("player").expect("shell state should initialize");

        let output = command.execute(&[], &mut state);

        assert!(matches!(output, CommandOutput::CLEAN));
    }

    #[test]
    fn clear_registers_cls_alias() {
        let command = register_command();

        assert_eq!(command.get_alias(), vec!["cls".to_string()]);
    }
}
