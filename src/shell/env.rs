use std::env;

pub fn get_home() -> Result<String, env::VarError> {
    let home = if cfg!(target_os = "windows") {
        "USERPROFILE"
    } else {
        "HOME"
    };

    env::var(home)
}
