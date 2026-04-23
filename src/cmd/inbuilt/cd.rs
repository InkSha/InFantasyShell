use crate::{cmd::command, system::error::FsError, system::storage::node::NodeId};

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("cd");

    cmd.with_casesensitive(false).with_handle(|args, system| {
        let target = args.first().map(String::as_str).unwrap_or("");

        let next_dir: Result<NodeId, FsError> = if target.is_empty() {
            match system.home {
                Some(home_id) => Ok(home_id),
                None => Err(FsError::NotFound("home directory not set".to_string())),
            }
        } else {
            system
                .storage
                .change_dir(system.get_cwd(), target, system.get_actor().as_str())
        };

        match next_dir {
            Ok(node_id) => {
                system.change_cwd(node_id);
                command::CommandOutput::NONE
            }
            Err(error) => command::CommandOutput::ERROR(error.to_string()),
        }
    });

    cmd
}
