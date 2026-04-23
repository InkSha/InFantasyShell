use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("clear");

    cmd.with_casesensitive(false)
        .with_handle(|_, _| command::CommandOutput::CLEAN);

    cmd
}
