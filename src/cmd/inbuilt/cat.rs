use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("cat");

    cmd.with_casesensitive(false).with_handle(|args, state| {
        let Some(path) = args.first() else {
            return command::CommandOutput::ERROR("cat requires a path".to_string());
        };

        match state
            .system
            .storage
            .read_file(state.cwd, path.as_str(), state.actor.as_str())
        {
            Ok(content) => command::CommandOutput::DISPLAY(content),
            Err(error) => command::CommandOutput::ERROR(error.to_string()),
        }
    });

    cmd
}
