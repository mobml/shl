use super::ShellError;

pub struct Parser;

#[derive(Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        Parser
    }
    pub fn parse(&self, input: &str) -> Result<ParsedCommand, ShellError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ShellError::Parse("Input cannot be empty".to_string()));
        }

        let mut parts = input.split_whitespace();
        let name = parts
            .next()
            .ok_or_else(|| ShellError::Parse("No command name founded".to_string()))?
            .to_string();

        let args = parts.map(|arg| arg.to_string()).collect();

        Ok(ParsedCommand { name, args })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_command_with_args() {
        let parser = Parser;

        let input = "echo Hello, Rust!";
        let result = parser.parse(input);

        assert!(result.is_ok());
        let parsed_command = result.unwrap();
        assert_eq!(parsed_command.name, "echo");
        assert_eq!(parsed_command.args, vec!["Hello,", "Rust!"]);
    }

    #[test]
    fn test_parse_valid_command_without_args() {
        let parser = Parser;

        let input = "ls";
        let result = parser.parse(input);

        assert!(result.is_ok());
        let parsed_command = result.unwrap();
        assert_eq!(parsed_command.name, "ls");
        assert_eq!(parsed_command.args, Vec::<String>::new());
    }

    #[test]
    fn test_parse_empty_input() {
        let parser = Parser;

        let input = "";
        let result = parser.parse(input);

        assert!(result.is_err());
        if let Err(ShellError::Parse(err)) = result {
            assert_eq!(err, "Input cannot be empty");
        } else {
            panic!("Expected ShellError::Parse");
        }
    }

    #[test]
    fn test_parse_whitespace_only_input() {
        let parser = Parser;

        let input = "   ";
        let result = parser.parse(input);

        assert!(result.is_err());
        if let Err(ShellError::Parse(err)) = result {
            assert_eq!(err, "Input cannot be empty");
        } else {
            panic!("Expected ShellError::Parse");
        }
    }

    #[test]
    fn test_parse_input_with_only_command_name() {
        let parser = Parser;

        let input = "cd";
        let result = parser.parse(input);

        assert!(result.is_ok());
        let parsed_command = result.unwrap();
        assert_eq!(parsed_command.name, "cd");
        assert_eq!(parsed_command.args, Vec::<String>::new());
    }

    #[test]
    fn test_parse_command_with_extra_spaces() {
        let parser = Parser;

        let input = "   mkdir    new_folder   ";
        let result = parser.parse(input);

        assert!(result.is_ok());
        let parsed_command = result.unwrap();
        assert_eq!(parsed_command.name, "mkdir");
        assert_eq!(parsed_command.args, vec!["new_folder"]);
    }
}
