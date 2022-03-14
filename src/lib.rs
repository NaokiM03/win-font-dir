use std::{env, path::PathBuf};

/// Returns the path to user's font directory.
pub fn user_font_dir() -> Option<PathBuf> {
    env::var_os("LOCALAPPDATA").and_then(|x| {
        let path = PathBuf::from(x).join("Microsoft\\Windows\\Fonts");
        if path.is_dir() {
            Some(path)
        } else {
            None
        }
    })
}

/// Returns the path to system's font directory.
pub fn system_font_dir() -> Option<PathBuf> {
    env::var_os("windir").and_then(|x| {
        let path = PathBuf::from(x).join("Fonts");
        if path.is_dir() {
            Some(path)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{system_font_dir, user_font_dir};

    #[test]
    fn test() {
        println!(
            "user font dir:   {}",
            user_font_dir().unwrap().to_str().unwrap()
        );
        println!(
            "system font dir: {}",
            system_font_dir().unwrap().to_str().unwrap()
        );
    }
}
