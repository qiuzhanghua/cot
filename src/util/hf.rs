use anyhow::{Context, Result};
use log::debug;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn get_metadata(path: &str) -> Result<fs::Metadata> {
    fs::metadata(path).with_context(|| format!("Failed to get metadata for {}", path))
}

fn is_dir(metadata: fs::Metadata) -> Result<()> {
    if !metadata.is_dir() {
        return Err(anyhow::anyhow!("Not a directory"));
    }
    Ok(())
}

fn expand_path(default_dir: &str) -> Result<std::path::PathBuf> {
    let mut expanded_dir = std::path::PathBuf::from(default_dir);
    if let Some(stripped) = default_dir.strip_prefix('~') {
        let home_dir = env::var("HOME").with_context(|| "Failed to get HOME directory")?;
        expanded_dir = std::path::PathBuf::from(home_dir);
        expanded_dir.push(stripped);
    }
    Ok(expanded_dir)
}

fn get_dir_with_env(env_var: &str, default_dir: &str) -> Result<String> {
    // Try to get the environment variable value
    if let Ok(result) = env::var(env_var) {
        is_dir(get_metadata(&result)?)?;
        return Ok(result);
    }

    // If the environment variable is not set, try to expand the default directory
    let expanded_dir = expand_path(default_dir)?;
    is_dir(get_metadata(expanded_dir.as_path().to_str().unwrap_or(""))?)?;
    Ok(expanded_dir.to_str().unwrap_or("").to_string())
}

pub fn xdg_cache_home() -> Result<String> {
    get_dir_with_env("XDG_CACHE_HOME", "~/.cache")
}

pub fn hf_home() -> Result<String> {
    get_dir_with_env("HF_HOME", "~/.cache/huggingface")
}

pub fn huggingface_hub_cache() -> Result<String> {
    let cache = if env::var("HUGGINGFACE_HUB_CACHE").is_ok() {
        get_dir_with_env("HUGGINGFACE_HUB_CACHE", "~/.cache/huggingface/hub")?
    } else {
        let hf_home_path = hf_home()?;
        let mut cache_path = std::path::PathBuf::from(hf_home_path);
        cache_path.push("hub");
        cache_path.to_str().unwrap_or("").to_string()
    };

    debug!("Huggingface Hub Cache: {}", cache);

    let metadata = get_metadata(&cache)?;
    if !metadata.is_dir() {
        return Err(anyhow::anyhow!("{} is not a directory", cache));
    }

    Ok(cache)
}

pub fn hf_datasets_cache() -> Result<String> {
    let cache = if env::var("HUGGINGFACE_HUB_CACHE").is_ok() {
        get_dir_with_env("HF_DATASETS_CACHE", "~/.cache/huggingface/datasets")?
    } else {
        let mut cache_path = PathBuf::from(hf_home()?);
        cache_path.push("datasets");
        cache_path.to_str().unwrap_or("").to_string()
    };

    debug!("Huggingface Datasets Cache: {}", cache);

    let metadata = get_metadata(&cache)?;
    if !metadata.is_dir() {
        return Err(anyhow::anyhow!("{} is not a directory", cache));
    }

    Ok(cache)
}

pub fn hf_model_path(model: &str) -> Result<String> {
    let cache = huggingface_hub_cache()?;
    let model_path = format!("models--{}", model.replace("/", "--"));
    let model_dir = PathBuf::from(cache).join(model_path);

    let result = model_dir.join("snapshots").join(read_oid_of(&model_dir)?);

    let meta_data = fs::metadata(&result)?;
    if !meta_data.is_dir() {
        return Err(anyhow::anyhow!("{} is not a directory", result.display()));
    }

    Ok(result.to_str().unwrap_or("").to_string())
}
pub fn hf_datasets_path(model: &str) -> Result<String> {
    let cache = huggingface_hub_cache()?;
    let ds_path = format!("datasets--{}", model.replace("/", "--"));
    let ds_dir = PathBuf::from(cache).join(ds_path);

    let result = ds_dir.join("snapshots").join(read_oid_of(&ds_dir)?);

    let meta_data = fs::metadata(&result)?;
    if !meta_data.is_dir() {
        return Err(anyhow::anyhow!("{} is not a directory", result.display()));
    }

    Ok(result.to_str().unwrap_or("").to_string())
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
