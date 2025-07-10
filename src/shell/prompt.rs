use std::env;

use super::{env::get_home, errors::ShellError};

pub struct Prompt {
    path: String,
}

impl Prompt {
    pub fn set_path(&mut self) -> Result<(), ShellError> {
        let path = env::current_dir()?;
        let home = get_home().map_err(|_| ShellError::HomeDirNotFound)?;

        let path_str = path.to_string_lossy();

        if path_str == home {
            self.path = String::from("\x1b[32m$ \x1b[0m");
        } else {
            self.path = String::from(path_str);
            self.path.push_str("\x1b[32m $ \x1b[0m");
        }
        Ok(())
    }
    pub fn new() -> Result<Self, ShellError> {
        let mut prompt = Prompt {
            path: String::new(),
        };
        prompt.set_path()?;
        Ok(prompt)
    }
    pub fn get_path(&self) -> &String {
        &self.path
    }
}
