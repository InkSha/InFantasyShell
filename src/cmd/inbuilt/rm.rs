use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("rm");

    cmd.with_casesensitive(false).with_handle(|args, system| {
        let Some(path) = args.first() else {
            return command::CommandOutput::ERROR("rm requires a path".to_string());
        };

        match system
            .storage
            .remove(system.get_cwd(), path.as_str(), &system.get_actor())
        {
            Ok(()) => command::CommandOutput::NONE,
            Err(error) => command::CommandOutput::ERROR(error.to_string()),
        }
    });

    cmd
}
