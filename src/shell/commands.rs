use super::errors::ShellError;
use std::env;
use std::io::Write;
use std::path::PathBuf;

pub trait Command {
    fn execute(&self, args: &[String], stdout: &mut dyn Write) -> Result<(), ShellError>;
}

pub struct HelloCommand;
pub struct PwdCommand;
pub struct ExitCommand;
pub struct CdCommand;

impl HelloCommand {
    pub fn new() -> Self {
        HelloCommand
    }
}

impl PwdCommand {
    pub fn new() -> Self {
        PwdCommand
    }
}

impl ExitCommand {
    pub fn new() -> Self {
        ExitCommand
    }
}

impl CdCommand {
    pub fn new() -> Self {
        CdCommand
    }
}

/// Implementation of the `Command` trait for the commands

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

impl Command for PwdCommand {
    fn execute(&self, _args: &[String], stdout: &mut dyn Write) -> Result<(), ShellError> {
        let path = env::current_dir()?;
        let display_path = path
            .into_os_string()
            .into_string()
            .unwrap_or_else(|os| os.to_string_lossy().into_owned());
        writeln!(stdout, "{display_path}")?;
        Ok(())
    }
}

impl Command for CdCommand {
    fn execute(&self, args: &[String], stdout: &mut dyn Write) -> Result<(), ShellError> {
        if args.len() == 0 {
            return Err(ShellError::ArgNotFound("cd".to_string()));
        }
        let pwd = env::current_dir()?;
        let mut path = PathBuf::from(pwd);
        let name = &args[0];
        path.push(name);

        if !path.is_dir() {
            return Err(ShellError::DirNotFound(name.to_string()));
        }

        env::set_current_dir(&path)?;
        if let Some(s) = path.to_str() {
            writeln!(stdout, "{s}")?
        }

        Ok(())
    }
}
