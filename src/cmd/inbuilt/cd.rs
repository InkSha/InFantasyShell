use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("cd");

    cmd.with_casesensitive(false).with_handle(|args, state| {
        let target = args.first().map(String::as_str).unwrap_or("");
        let next_dir = if target.is_empty() {
            Ok(state.home)
        } else {
            state
                .vfs
                .change_dir(state.cwd, target, state.actor.as_str())
        };

        match next_dir {
            Ok(node_id) => {
                state.cwd = node_id;
                command::CommandOutput::NONE
            }
            Err(error) => command::CommandOutput::ERROR(error.to_string()),
        }
    });

    cmd
}
