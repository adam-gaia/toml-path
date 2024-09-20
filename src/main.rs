use clap::Parser;
use color_eyre::eyre::bail;
use color_eyre::Result;
use log::debug;
use std::fs;
use std::io::BufRead;
use std::io::{self};
use std::path::PathBuf;
use toml::Value;
use toml_path::get;
use toml_path::Settings;
use toml_path::TomlPath;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// Toml path
    path: TomlPath,

    /// Toml file to process. Toml content is read from stdin if omitted
    file: Option<PathBuf>,

    /// Write output directly, rather than formatting as toml. Useful for displaying strings without quotes
    #[arg(short, long, group = "output_format")]
    raw_output: bool, // TODO: should we break "jq compatibility" (not that we are in any way comatible lol) and default to true? When do I ever not pass this flag?

    /// Print output to a single line instead of pretty-printing to multiple lines
    #[arg(short, long)]
    compact_output: bool,

    /// Output as json
    #[arg(short, long, group = "output_format")]
    json_output: bool, // TODO: add more flags like jq
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let args = Cli::parse();
    debug!("args: {:?}", args);

    let settings = Settings::builder()
        .raw_output(args.raw_output)
        .compact_output(args.compact_output)
        .json_output(args.json_output)
        .build();
    debug!("settings: {:?}", settings);

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
    let result = get(&toml, &args.path, &settings)?;
    println!("{}", result);

    Ok(())
}
