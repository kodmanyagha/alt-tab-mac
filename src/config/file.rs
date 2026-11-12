// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs;
use std::path::{Path, PathBuf};

use super::Config;
use std::io;

const APP_DIR: &str = "alt-tab-mac";
const FILE_NAME: &str = "config.toml";

/// `~/Library/Application Support/alt-tab-mac/config.toml`
pub fn config_path() -> PathBuf {
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_default();
    home.join("Library/Application Support").join(APP_DIR).join(FILE_NAME)
}

/// Writes the defaults if the file is missing. If the file is broken, continues with the defaults without overwriting
/// the user's file.
pub fn load_or_create() -> Config {
    let path = config_path();
    match fs::read_to_string(&path) {
        Ok(text) => parse_or_default(&text),
        Err(_) => create_default(&path),
    }
}

fn parse_or_default(text: &str) -> Config {
    toml::from_str(text).unwrap_or_else(|err| {
        tracing::error!(error = %err, "could not read config.toml, using defaults");
        Config::default()
    })
}

/// Writes the settings to disk (on every change in the settings window).
pub fn save(config: &Config) {
    if let Err(err) = write(&config_path(), config) {
        tracing::error!(error = %err, "could not write config.toml");
    }
}

fn create_default(path: &Path) -> Config {
    let config = Config::default();
    if let Err(err) = write(path, &config) {
        tracing::warn!(error = %err, "could not write default config");
    }
    config
}

fn write(path: &Path, config: &Config) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let text = toml::to_string_pretty(config).map_err(io::Error::other)?;
    fs::write(path, text)
}
