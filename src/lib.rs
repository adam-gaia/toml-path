#[doc = include_str!("../README.md")]
use eyre::Result;
use log::debug;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use toml::Value;

mod toml_path;
pub use toml_path::{Index, Op, TomlPath};

mod traverse;
use traverse::traverse;

mod settings;
pub use settings::Settings;

mod format;
use format::format_value;

/// Get value(s) specified by a tomlpath from a toml
pub fn get(toml: &Value, path: &TomlPath, settings: &Settings) -> Result<String> {
    let value = traverse(&toml, &path.parts())?;
    Ok(format_value(&value, &settings))
}

/// Convenience wrapper for the [get] function to get a value directly from a file.
/// Uses default values for [Settings].
/// For more flexibility, see [get], which allows configuration at the cost of convenience.
pub fn get_from_file<P: AsRef<Path>>(file: P, tomlpath: &str) -> Result<String> {
    let file = fs::canonicalize(file)?;
    debug!("Reading file: {}", file.display());
    let contents = fs::read_to_string(file)?;
    let toml: Value = toml::from_str(&contents)?;
    let toml_path = TomlPath::from_str(tomlpath)?;
    let settings = Settings::default();
    let result = get(&toml, &toml_path, &settings)?;
    Ok(result)
}
