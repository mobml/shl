use parser::Parser;
use registry::CommandRegistry;
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
        return Shl {
            parser: Parser::new(),
            registry: CommandRegistry::new(),
        };
    }

    //TODO
    // Separate function to initialize the register     [_]
    // Implement the remaining Commands                 [_]
    // Handle errors properly                           [_]

    pub fn run(&mut self) -> Result<(), ShellError> {
        self.registry
            .register("hello".to_string(), commands::HelloWorld::new());
        let prompt = "\x1b[32m$ \x1b[0m";
        let mut input = String::new();
        loop {
            input.clear();
            io::stdout()
                .write_all(prompt.as_bytes())
                .map_err(ShellError::from)?;

            io::stdout().flush().map_err(ShellError::from)?;

            io::stdin()
                .read_line(&mut input)
                .map_err(ShellError::from)?;

            io::stdout()
                .write_all(&input.as_bytes())
                .map_err(ShellError::from)?;

            match self.parser.parse(&input) {
                Ok(command) => {
                    println!("{:?}", &command);
                    let _ = self.registry.command_founded(command);
                }
                Err(_) => eprintln!("Something wrong"),
            }

            if input.trim() == "exit" {
                break;
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
}

impl From<io::Error> for ShellError {
    fn from(err: io::Error) -> Self {
        ShellError::IO(err)
    }
}
