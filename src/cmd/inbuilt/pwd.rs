use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("pwd");

    cmd.with_casesensitive(false).with_handle(|_, system| {
        match system.storage.absolute_path(system.get_cwd()) {
            Ok(path) => command::CommandOutput::DISPLAY(path),
            Err(error) => command::CommandOutput::ERROR(error.to_string()),
        }
    });

    cmd
}
