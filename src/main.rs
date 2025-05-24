use std::env;
use std::io::{self, Write};

/// Entry point of the program
/// Call the function that contains the shell main loop
fn main() {
    shl_loop().expect("Error in the main loop of the shell");
}

/// Main loop of the shell
/// Displays the prompt, reads the user's input
/// and analizes and execute commands.
///
/// # Errors
///
/// Returns `io::Result<()>` if there is an error reading the current directory,
/// writing the prompt or reading the user input.
fn shl_loop() -> io::Result<()> {
    loop {
        // Gets the current working directory
        let current_dir = env::current_dir().expect("Error reading the current directory");
        // Formats the prompt with colors: green for the directory and blue for the `~` simbol
        let promp = format!(
            "\x1b[32m{}\x1b[0m:\x1b[34m~\x1b[0m$ ",
            current_dir.display()
        );
        // Writes the prompt to the standard output
        io::stdout()
            .write_all(promp.as_bytes())
            .expect("Error to show line");

        // Enssures the prompt is displayed immediatly
        io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        // Reads input from the user and saves it in `input`
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }
    }
    return Ok(());
}
