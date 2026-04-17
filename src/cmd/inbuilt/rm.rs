use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("rm");

    cmd.with_casesensitive(false).with_handle(|args, state| {
        let Some(path) = args.first() else {
            return command::CommandOutput::ERROR("rm requires a path".to_string());
        };

        match state.vfs.remove(state.cwd, path.as_str(), state.actor.as_str()) {
            Ok(()) => command::CommandOutput::NONE,
            Err(error) => command::CommandOutput::ERROR(error.to_string()),
        }
    });

    cmd
}
