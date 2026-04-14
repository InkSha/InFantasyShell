use std::{collections::HashMap, sync::Arc};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub mod command;
pub mod context;
pub mod inbuilt;
mod parser;

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
        ctx.set_prompt(self.prompt.to_string());

        ctx.write_with_prompt("");

        loop {
            ctx.read();

            if ctx.is_enter() {
                let line = ctx.read_line();

                match parser::parse(line.trim().into()) {
                    Ok((cmd, args)) => {
                        if let Some(command) = self.commands.get(&cmd) {
                            match command.execute(args) {
                                // TODO: implement reactive commands
                                command::CommandOutput::REACTIVE => (),
                                command::CommandOutput::DISPLAY(output) => ctx.write(output),
                                command::CommandOutput::EXIT(msg) => {
                                    if !msg.is_empty() {
                                        ctx.write(msg);
                                    }
                                    break;
                                }
                                command::CommandOutput::ERROR(msg) => {
                                    ctx.write(format!("\nError: {}", msg));
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
                ctx.new_line();
            } else {
                ctx.write_with_prompt(ctx.read_line());
            }
        }

        disable_raw_mode().unwrap();
    }
}
