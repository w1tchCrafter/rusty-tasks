use std::fs;
use std::path::Path;

use dirs;

fn home_dir() -> Result<String, String> {
    match dirs::home_dir() {
        Some(path) => {
            match path.to_str() {
                Some(home_dir_str) => Ok(home_dir_str.to_string()),
                None => Err("failed to convert home dir to string".to_string())
            }
        }
        None => Err("error getting home directory".to_string())
    }
}

pub fn setup() -> std::io::Result<()>  {
    match home_dir() {
        Ok(path_str) => {
            let path = Path::new(&path_str);

            if !path.is_dir() || !path.exists() {
                fs::create_dir(format!("{}", path_str))?
            }
        },
        Err(_) => panic!("error: cannot create file for saving app data")
    }
    Ok(())
}
