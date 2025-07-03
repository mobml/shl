use std::fmt::Display;
use std::io;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ShellError {
    IO(io::Error),
    Parse(String),
    CommandNotFound(String),
    Exit,
}

impl Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::IO(err) => write!(f, "I/O error: {}", err),
            ShellError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ShellError::CommandNotFound(cmd) => write!(f, "Command \"{}\" not found", cmd),
            ShellError::Exit => write!(f, "Exiting shell"),
        }
    }
}

impl From<io::Error> for ShellError {
    fn from(err: io::Error) -> Self {
        ShellError::IO(err)
    }
}
