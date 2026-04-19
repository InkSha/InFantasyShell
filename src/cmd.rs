use std::{collections::HashMap, sync::Arc};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub mod command;
pub mod context;
pub mod inbuilt;
mod parser;
pub mod runtime;

pub struct Cmd {
    pub commands: HashMap<String, Arc<command::Command>>,
    pub prompt: String,
}

impl Cmd {
    pub fn new<T: ToString>(prompt: T) -> Self {
        Self {
            prompt: prompt.to_string(),
            commands: inbuilt::get_commands(),
        }
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();

        let mut ctx = context::Context::default();
        let mut shell_state = runtime::ShellState::new("player")
            .expect("shell state should initialize with the default VFS world");
        ctx.set_prompt(shell_state.render_prompt(self.prompt.as_str()));

        ctx.clear_screen();
        ctx.write_with_prompt("");

        loop {
            ctx.read();

            if !ctx.is_enter() {
                ctx.write_with_prompt(ctx.read_line());
                continue;
            }

            let line = ctx.read_line();
            let trimmed = line.trim();

            if trimmed.is_empty() {
                ctx.set_prompt(shell_state.render_prompt(self.prompt.as_str()));
                ctx.new_line();
                continue;
            }

            ctx.write_with_prompt(ctx.read_line());

            match parser::parse(trimmed) {
                Ok((cmd, args)) => {
                    let normalized = cmd.to_lowercase();

                    if let Some(command) = self
                        .commands
                        .get(&cmd)
                        .or_else(|| self.commands.get(&normalized))
                    {
                        match command.execute(&args, &mut shell_state) {
                            // TODO: implement reactive commands
                            command::CommandOutput::REACTIVE => (),
                            command::CommandOutput::DISPLAY(output) => {
                                ctx.write(format!("{}", output))
                            }
                            command::CommandOutput::EXIT(msg) => {
                                if !msg.is_empty() {
                                    ctx.write(format!("{}", msg));
                                }
                                break;
                            }
                            command::CommandOutput::CLEAN => {
                                ctx.clear_screen();
                            }
                            command::CommandOutput::ERROR(msg) => {
                                ctx.write(format!("Error: {}", msg));
                            }
                            // NOOP
                            command::CommandOutput::NONE => {}
                        }
                    } else {
                        ctx.write(format!("Unknown command: {}", cmd));
                    }
                }
                Err(msg) => {
                    ctx.write(format!("Error: {}", msg));
                }
            }
            ctx.set_prompt(shell_state.render_prompt(self.prompt.as_str()));
            ctx.new_line();
        }

        disable_raw_mode().unwrap();
    }
}
