use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use log;

fn get_dir_with_env(env_var: &str, default_dir: &str) -> Result<String, io::Error> {
    // Try to get the environment variable value
    if let Ok(result) = env::var(env_var) {
        // Check if the path is a directory
        let file_info = match fs::metadata(&result) {
            Ok(info) => info,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };

        return if file_info.is_dir() {
            Ok(result)
        } else {
            log::error!("{} is not a directory", result);
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a directory", result),
            ))
        }
    }

    // If the environment variable is not set, try to expand the default directory
    let mut expanded_dir = PathBuf::from(default_dir);
    if default_dir.starts_with('~') {
        let home_dir = match env::var("HOME") {
            Ok(path) => path,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed to get HOME directory")),
        };
        expanded_dir = PathBuf::from(home_dir);
        expanded_dir.push(&default_dir[1..]);
    }

    // Check if the expanded default directory is a valid directory
    let file_info = match fs::metadata(expanded_dir.as_path()) {
        Ok(info) => info,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
    };

    if file_info.is_dir() {
        return Ok(expanded_dir.to_str().unwrap_or("").to_string());
    } else {
        log::error!("{} is not a directory", default_dir);
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a directory", default_dir),
        ));
    }
}

pub fn xdg_cache_home() -> Result<String, io::Error> {
    get_dir_with_env("XDG_CACHE_HOME", "~/.cache")
}

pub fn hf_home() -> Result<String, io::Error> {
    get_dir_with_env("HF_HOME", "~/.cache/huggingface")
}

