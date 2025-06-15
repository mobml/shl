use super::commands;
use super::commands::Command;
use super::{ShellError, parser::ParsedCommand};
use std::collections::HashMap;
use std::io::Write;

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
    pub fn run_command(
        &self,
        parsed_cmd: ParsedCommand,
        stdout: &mut dyn Write,
    ) -> Result<(), ShellError> {
        match self.commands.get(&parsed_cmd.name) {
            Some(cmd) => cmd.execute(&parsed_cmd.args, stdout),
            None => Err(ShellError::CommandNotFound(parsed_cmd.name.to_string())),
        }
    }

    pub fn init_commands(&mut self) {
        self.register("hello".to_string(), commands::HelloCommand::new());
        self.register("exit".to_string(), commands::ExitCommand::new());
    }
}
