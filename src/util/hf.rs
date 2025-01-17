use log::debug;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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
        };
    }

    // If the environment variable is not set, try to expand the default directory
    let mut expanded_dir = PathBuf::from(default_dir);
    if let Some(stripped) = default_dir.strip_prefix('~') {
        let home_dir = match env::var("HOME") {
            Ok(path) => path,
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to get HOME directory",
                ))
            }
        };
        expanded_dir = PathBuf::from(home_dir);
        expanded_dir.push(stripped);
    }

    // Check if the expanded default directory is a valid directory
    let file_info = match fs::metadata(expanded_dir.as_path()) {
        Ok(info) => info,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
    };

    if file_info.is_dir() {
        Ok(expanded_dir.to_str().unwrap_or("").to_string())
    } else {
        log::error!("{} is not a directory", default_dir);
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a directory", default_dir),
        ))
    }
}

pub fn xdg_cache_home() -> Result<String, io::Error> {
    get_dir_with_env("XDG_CACHE_HOME", "~/.cache")
}

pub fn hf_home() -> Result<String, io::Error> {
    get_dir_with_env("HF_HOME", "~/.cache/huggingface")
}

pub fn huggingface_hub_cache() -> Result<String, io::Error> {
    let cache;

    if env::var("HUGGINGFACE_HUB_CACHE").is_ok() {
        match get_dir_with_env("HUGGINGFACE_HUB_CACHE", "~/.cache/huggingface/hub") {
            Ok(path) => cache = path.clone(),
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    } else {
        match hf_home() {
            Ok(hf_home_path) => {
                let mut cache_path = PathBuf::from(hf_home_path);
                cache_path.push("hub");
                cache = cache_path.to_str().unwrap_or("").to_string();
            }
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }

    debug!("Huggingface Hub Cache: {}", cache);

    match fs::metadata(&cache) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(cache)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotADirectory,
                    format!("{} is not a directory", cache),
                ))
            }
        }
        Err(err) => Err(err),
    }
}

pub fn hf_datasets_cache() -> Result<String, io::Error> {
    let cache;

    if env::var("HUGGINGFACE_HUB_CACHE").is_ok() {
        match get_dir_with_env("HF_DATASETS_CACHE", "~/.cache/huggingface/datasets") {
            Ok(path) => cache = path.clone(),
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    } else {
        match hf_home() {
            Ok(hf_home_path) => {
                let mut cache_path = PathBuf::from(hf_home_path);
                cache_path.push("datasets");
                cache = cache_path.to_str().unwrap_or("").to_string();
            }
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }

    debug!("Huggingface Datasets Cache: {}", cache);

    match fs::metadata(&cache) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(cache)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotADirectory,
                    format!("{} is not a directory", cache),
                ))
            }
        }
        Err(err) => Err(err),
    }
}

pub fn hf_model_path(model: &str) -> Result<String, io::Error> {
    let cache = match huggingface_hub_cache() {
        Ok(cache) => cache,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
    };

    let model_path = format!("models--{}", model.replace("/", "--"));
    let model_dir = PathBuf::from(cache).join(model_path);

    let oid = match read_oid_of(&model_dir) {
        Ok(oid) => oid,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
    };

    let result = model_dir.join("snapshots").join(oid);
    match fs::metadata(&result) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(result.to_str().unwrap_or("").to_string())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotADirectory,
                    format!("{} is not a directory", result.display()),
                ))
            }
        }
        Err(err) => Err(err),
    }
}

pub fn hf_datasets_path(model: &str) -> Result<String, io::Error> {
    let cache = match huggingface_hub_cache() {
        Ok(cache) => cache,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
    };

    let ds_path = format!("datasets--{}", model.replace("/", "--"));
    let ds_dir = PathBuf::from(cache).join(ds_path);

    let oid = match read_oid_of(&ds_dir) {
        Ok(oid) => oid,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
    };

    let result = ds_dir.join("snapshots").join(oid);
    match fs::metadata(&result) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(result.to_str().unwrap_or("").to_string())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::NotADirectory,
                    format!("{} is not a directory", result.display()),
                ))
            }
        }
        Err(err) => Err(err),
    }
}
fn read_oid_of(model_or_ds: &Path) -> Result<String, io::Error> {
    let file_path = model_or_ds.join("refs").join("main");
    match fs::read_to_string(file_path.clone()) {
        Ok(data) => Ok(data.trim().to_string()),
        Err(err) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read OID from {}: {}", file_path.display(), err),
        )),
    }
}
