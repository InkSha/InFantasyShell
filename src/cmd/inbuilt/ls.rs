use crate::cmd::{command, output::format};
use crate::system::storage::node::NodeType;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("ls");

    cmd.with_casesensitive(false).with_handle(|args, system| {
        let path: &str = args.first().map(String::as_str).unwrap_or(".");
        match system
            .storage
            .list_dir(system.get_cwd(), path, &system.get_actor())
        {
            Ok(entries) => {
                let items = entries
                    .into_iter()
                    .map(|entry| {
                        let suffix = match entry.node_type {
                            NodeType::Directory => "/",
                            NodeType::File => "",
                        };
                        format!("{}{}", entry.name, suffix)
                    })
                    .collect::<Vec<_>>();

                let output = format::rows_output(items);

                command::CommandOutput::DISPLAY(output)
            }
            Err(error) => command::CommandOutput::ERROR(error.to_string()),
        }
    });

    cmd
}
