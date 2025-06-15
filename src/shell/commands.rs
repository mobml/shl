use std::io::Write;

use super::ShellError;

pub trait Command {
    fn execute(&self, args: &[String], stdout: &mut dyn Write) -> Result<(), ShellError>;
}

pub struct HelloCommand;

impl HelloCommand {
    pub fn new() -> Self {
        HelloCommand
    }
}

pub struct ExitCommand;

impl ExitCommand {
    pub fn new() -> Self {
        ExitCommand
    }
}

impl Command for HelloCommand {
    fn execute(&self, args: &[String], stdout: &mut dyn Write) -> Result<(), ShellError> {
        if let Some(name) = args.first() {
            writeln!(stdout, "Hello, {}!!!", name)?;
        } else {
            writeln!(stdout, "Hello!!!")?;
        }
        Ok(())
    }
}

impl Command for ExitCommand {
    fn execute(&self, _args: &[String], _stdout: &mut dyn Write) -> Result<(), ShellError> {
        Err(ShellError::Exit)
    }
}
