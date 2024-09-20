use clap::{Parser, Subcommand};
use color_eyre::eyre::bail;
use color_eyre::Result;
use log::debug;
use log::info;
use std::fs;
use std::io::BufRead;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use toml::Table;
use toml::Value;
use toml_path::get;
use toml_path::TomlPath;

// TODO: add tests in README with trycmd

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// Toml path
    path: TomlPath,

    /// Toml file to process. Toml content is read from stdin if omitted
    file: Option<PathBuf>,
    // TODO: add args like jq's '--raw-input' and '--raw-output'
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let args = Cli::parse();

    let input = match args.file {
        Some(file) => {
            let file = file.canonicalize()?;
            let contents = fs::read_to_string(file)?;
            contents
        }
        None => {
            // Read toml from stdin
            let mut input = String::new();
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(content) => {
                        input.push_str(&content);
                    }
                    Err(e) => {
                        bail!("Error reading stdin: {}", e);
                    }
                }
            }
            input
        }
    };

    let toml: Value = toml::from_str(&input)?;
    let result = get(&toml, &args.path)?;
    println!("{}", result);

    Ok(())
}
