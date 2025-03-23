const TUIFEED_VERSION: &str = env!("CARGO_PKG_VERSION");
const TUIFEED_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

// -- libs
#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::path::PathBuf;
use std::process::exit;

use argh::FromArgs;

// includes
mod config;
mod feed;
mod helpers;
mod ui;

// -- internal
use config::{Config, serializer as config_serializer};
use helpers::{file as file_helpers, open as open_helpers, path as path_helpers};
use ui::Ui;

#[derive(FromArgs)]
#[argh(description = "
Please, report issues to <https://github.com/veeso/tuifeed>
Please, consider supporting the author <https://www.buymeacoffee.com/veeso>")]
struct Args {
    #[argh(switch, short = 'c', description = "open tuifeed configuration")]
    config: bool,
    #[argh(
        option,
        short = 'T',
        default = "10",
        description = "set UI ticks; default 10ms"
    )]
    ticks: u64,
    #[argh(switch, short = 'v', description = "print version")]
    version: bool,
}

fn main() {
    let args: Args = argh::from_env();
    // Print version
    if args.version {
        println!(
            "tuifeed - {} - Developed by {}",
            TUIFEED_VERSION, TUIFEED_AUTHORS
        );
        exit(255);
    }
    // Open config file
    if args.config {
        if let Err(e) = edit_config_file() {
            eprintln!("{}", e);
            exit(255)
        } else {
            exit(0)
        }
    }
    // Get configuration
    let config = match init_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to initialize configuration: {}", e);
            eprintln!(
                "If necessary, you can run tuifeed with `-c` option to edit and eventually fix your configuration file"
            );
            exit(255);
        }
    };
    // Check if configured
    if config.sources.is_empty() {
        eprintln!("tuifeed must be configured first. Run `tuifeed -c`");
        exit(255);
    }
    // Run ui
    Ui::init(config, args.ticks).run();
}

/// Edit configuration file
fn edit_config_file() -> Result<(), String> {
    if let Some(p) = get_config_file() {
        if let Err(e) = open_helpers::open_text_file(p.as_path()) {
            eprintln!();
            Err(format!("Could not open configuration file: {}", e))
        } else {
            Ok(())
        }
    } else {
        Err("Could not find a configuration path on your operating system...".to_string())
    }
}

/// Initialize configuration
fn init_config() -> Result<Config, String> {
    let config_dir = path_helpers::init_config_dir()?;
    if config_dir.is_none() {
        return Ok(Config::default());
    }
    let config_file = path_helpers::get_config_file(config_dir.unwrap().as_path())?;
    // Parse configuration
    let config_file =
        file_helpers::open_file_read(config_file.as_path()).map_err(|e| e.to_string())?;
    config_serializer::deserialize(config_file).map_err(|e| e.to_string())
}

/// Get configuration file path
fn get_config_file() -> Option<PathBuf> {
    let config_dir = match path_helpers::init_config_dir() {
        Ok(Some(p)) => p,
        _ => return None,
    };
    match path_helpers::get_config_file(config_dir.as_path()) {
        Ok(p) => Some(p),
        _ => None,
    }
}
