use crate::cmd::command;

pub fn register_command() -> command::Command {
    let mut cmd = command::Command::new("echo");

    cmd.with_casesensitive(false).with_handle(|args, system| {
        if args.is_empty() {
            return command::CommandOutput::DISPLAY(String::new());
        }

        if let Some(redirect_index) = args.iter().position(|arg| arg == ">") {
            if redirect_index + 1 >= args.len() {
                return command::CommandOutput::ERROR(
                    "echo redirection requires a target path".to_string(),
                );
            }

            let content = args[..redirect_index].join(" ");
            let path = &args[redirect_index + 1];

            return match system.storage.write_file(
                system.get_cwd(),
                path.as_str(),
                &system.get_actor(),
                content,
            ) {
                Ok(()) => command::CommandOutput::NONE,
                Err(error) => command::CommandOutput::ERROR(error.to_string()),
            };
        }

        command::CommandOutput::DISPLAY(args.join(" "))
    });

    cmd
}
