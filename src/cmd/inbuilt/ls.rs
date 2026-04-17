use crate::cmd::command;
use crate::vfs::NodeType;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("ls");

    cmd.with_casesensitive(false).with_handle(|args, state| {
        let path = args.first().map(String::as_str).unwrap_or(".");
        match state.vfs.list_dir(state.cwd, path, state.actor.as_str()) {
            Ok(entries) => {
                let output = entries
                    .into_iter()
                    .map(|entry| {
                        let suffix = match entry.node_type {
                            NodeType::Directory => "/",
                            NodeType::File => "",
                        };
                        format!("{}{} [{}] {}B", entry.name, suffix, entry.permissions, entry.size)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                command::CommandOutput::DISPLAY(output)
            }
            Err(error) => command::CommandOutput::ERROR(error.to_string()),
        }
    });

    cmd
}
