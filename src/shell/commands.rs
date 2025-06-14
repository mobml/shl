use super::ShellError;

pub trait Command {
    fn execute(&self, args: &Vec<String>) -> Result<(), ShellError>;
}

pub struct HelloWorld;

impl HelloWorld {
    pub fn new() -> Self {
        HelloWorld
    }
}

impl Command for HelloWorld {
    fn execute(&self, args: &Vec<String>) -> Result<(), ShellError> {
        println!("Hello, {:?}!", args[0]);
        Ok(())
    }
}
