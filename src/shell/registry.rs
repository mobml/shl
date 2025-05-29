use std::collections::HashMap;

pub struct CommandRegistry {
    commands: HashMap<String, fn()>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        return CommandRegistry {
            commands: HashMap::new(),
        };
    }
}
