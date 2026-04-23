use crate::system::System;

pub enum CommandOutput {
    NONE,
    CLEAN,
    DISPLAY(String),
    // TODO: implement reactive commands
    #[allow(dead_code)]
    REACTIVE,
    EXIT(String),
    ERROR(String),
}

pub type CommandHandle = fn(args: &[String], state: &mut System) -> CommandOutput;

pub fn command_default_handle(_: &[String], _: &mut System) -> CommandOutput {
    CommandOutput::NONE
}

pub struct Command {
    name: String,
    alias: Vec<String>,
    handle: CommandHandle,
    casesensitive: bool,
}

impl Command {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            alias: vec![],
            handle: command_default_handle,
            casesensitive: false,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_alias(&self) -> Vec<String> {
        self.alias.clone()
    }

    pub fn is_casesensitive(&self) -> bool {
        self.casesensitive
    }

    pub fn with_handle(&mut self, handle: CommandHandle) -> &mut Self {
        self.handle = handle;
        self
    }

    pub fn with_alias<T: ToString>(&mut self, alias: Vec<T>) -> &mut Self {
        self.alias = alias.into_iter().map(|a| a.to_string()).collect();
        self
    }

    pub fn with_casesensitive(&mut self, casesensitive: bool) -> &mut Self {
        self.casesensitive = casesensitive;
        self
    }

    pub fn execute(&self, args: &[String], state: &mut System) -> CommandOutput {
        (self.handle)(args, state)
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            name: String::new(),
            alias: vec![],
            handle: command_default_handle,
            casesensitive: false,
        }
    }
}
