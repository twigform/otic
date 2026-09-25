use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    #[serde(default)]
    dirs: Vec<PathBuf>,
}

fn pick_dir() -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .set_title("Select a folder")
        .pick_folder()
}

fn config_path() -> io::Result<PathBuf> {
    let base_dirs = BaseDirs::new()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "can't find a conf dir on ur os"))?;
    Ok(base_dirs.config_dir().join("otic").join("config.toml"))
}

fn conf_init() -> io::Result<PathBuf> {
    let path = config_path()?;
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        write_config(&path, &Config::default())?;
    }
    Ok(path)
}

fn read_config(path: &Path) -> io::Result<Config> {
    let contents = fs::read_to_string(path)?;
    toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn write_config(path: &Path, config: &Config) -> io::Result<()> {
    let toml_string = toml::to_string_pretty(config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, toml_string)
}

pub fn add_dir() -> io::Result<Option<PathBuf>> {
    let path = conf_init()?;
    let Some(chosen) = pick_dir() else {
        return Ok(None);
    };
    let mut config = read_config(&path)?;
    if config.dirs.contains(&chosen) {
        return Ok(None);
    }

    config.dirs.push(chosen.clone());
    write_config(&path, &config)?;

    Ok(Some(chosen))
}

pub fn rm_dir(index: usize) -> io::Result<()> {
    let path = conf_init()?;
    let mut config = read_config(&path)?;
    if index >= config.dirs.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "that dir number does not exist...?",
        ));
    }

    config.dirs.remove(index);
    write_config(&path, &config)?;

    Ok(())
}

pub fn list_dirs() -> io::Result<Vec<PathBuf>> {
    let path = conf_init()?;
    let config = read_config(&path)?;
    Ok(config.dirs)
}
