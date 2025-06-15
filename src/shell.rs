use parser::Parser;
use registry::CommandRegistry;
use std::fmt::Display;
use std::io::stdout;
use std::io::{self, Write};

mod commands;
mod parser;
mod registry;

pub struct Shl {
    parser: Parser,
    registry: CommandRegistry,
}

impl Shl {
    pub fn new() -> Self {
        Shl {
            parser: Parser::new(),
            registry: CommandRegistry::new(),
        }
    }

    //TODO
    // Separate function to initialize the register     [x]
    // Implement the remaining Commands                 [_]
    // Handle errors properly                           [_]

    pub fn run(&mut self) -> Result<(), ShellError> {
        self.registry.init_commands();
        let mut stdout = stdout();

        let prompt = "\x1b[32m$ \x1b[0m";
        let mut input = String::new();
        loop {
            input.clear();
            //show the prompt
            write!(stdout, "{prompt}")?;
            stdout.flush()?;
            // Read user input
            if io::stdin().read_line(&mut input)? == 0 {
                writeln!(stdout, "\nexit")?;
                break;
            }

            match self.parser.parse(&input) {
                Ok(command) => match self.registry.run_command(command, &mut stdout) {
                    Err(ShellError::Exit) => break,
                    Err(e) => writeln!(stdout, "Error: {e}")?,
                    _ => {}
                },
                Err(e) => writeln!(stdout, "Parse error: {e}")?,
            }
        }
        Ok(())
    }
}

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
