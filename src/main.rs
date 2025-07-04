use shell::errors::ShellError;

mod shell;

/// Entry point of the program
/// Call the function that contains the shell main loop
fn main() {
    let mut shl = shell::Shl::new();

    match shl.run() {
        Ok(_) => println!("run method successfully run"),
        Err(e) => match e {
            ShellError::Parse(str) => println!("{}", str),
            ShellError::IO(_) => eprintln!("Error I/O"),
            ShellError::CommandNotFound(str) => eprintln!("{}", str),
            ShellError::Exit => {}
            ShellError::ArgNotFound(_) => {}
            ShellError::DirNotFound(_) => {}
        },
    }
}
