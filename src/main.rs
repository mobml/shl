use shell::errors::ShellError;

mod shell;

/// Entry point of the program
/// Call the function that contains the shell main loop
fn main() -> Result<(), ShellError> {
    let mut shl = shell::Shl::new();
    shl.run()
}
