use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("exit");

    cmd.with_alias(vec!["quit"])
        .with_casesensitive(false)
        .with_handle(|_| {
            return command::CommandOutput::EXIT("".to_string());
        });

    return cmd;
}
