use super::commands;
use super::commands::Command;
use super::{ShellError, parser::ParsedCommand};
use std::collections::HashMap;

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn register<T>(&mut self, cmd_name: String, cmd: T)
    where
        T: Command + 'static,
    {
        self.commands.insert(cmd_name, Box::new(cmd));
    }
    pub fn new() -> Self {
        CommandRegistry {
            commands: HashMap::new(),
        }
    }
    pub fn command_founded(&self, parsed_cmd: ParsedCommand) -> Result<(), ShellError> {
        match self.commands.get(&parsed_cmd.name) {
            Some(cmd) => cmd.execute(&parsed_cmd.args),
            None => Err(ShellError::CommandNotFound(
                "Command not founded".to_string(),
            )),
        }
    }

    pub fn init_commands(&mut self) {
        self.register("hello".to_string(), commands::HelloWorld::new());
    }
}
