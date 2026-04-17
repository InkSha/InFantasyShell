use crate::cmd::command;
use crate::vfs::Permissions;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("chmod");

    cmd.with_casesensitive(false).with_handle(|args, state| {
        if args.len() < 2 {
            return command::CommandOutput::ERROR("chmod requires a mode and path".to_string());
        }

        let permissions = match Permissions::from_octal(args[0].as_str()) {
            Ok(permissions) => permissions,
            Err(error) => return command::CommandOutput::ERROR(error.to_string()),
        };

        match state.vfs.chmod(
            state.cwd,
            args[1].as_str(),
            state.actor.as_str(),
            permissions,
        ) {
            Ok(()) => command::CommandOutput::NONE,
            Err(error) => command::CommandOutput::ERROR(error.to_string()),
        }
    });

    cmd
}
