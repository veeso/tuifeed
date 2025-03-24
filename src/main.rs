const TUIFEED_VERSION: &str = env!("CARGO_PKG_VERSION");
const TUIFEED_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

// -- libs
#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::path::PathBuf;

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
Please, consider supporting the author <https://ko-fi.com/veeso>")]
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = argh::from_env();
    // Print version
    if args.version {
        println!(
            "tuifeed - {} - Developed by {}",
            TUIFEED_VERSION, TUIFEED_AUTHORS
        );
        return Ok(());
    }
    // Open config file
    if args.config {
        if let Err(e) = edit_config_file() {
            eprintln!("{}", e);
            return Err(e.into());
        } else {
            return Ok(());
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
            return Err(e.into());
        }
    };
    // Check if configured
    if config.sources.is_empty() {
        eprintln!("tuifeed must be configured first. Run `tuifeed -c`");
        return Err("Configuration is empty".into());
    }
    // Run ui
    Ui::init(config, args.ticks)?.run()
}

/// Edit configuration file
fn edit_config_file() -> Result<(), String> {
    let Some(p) = get_config_file() else {
        return Err("Could not find a configuration path on your operating system...".to_string());
    };

    if let Err(e) = open_helpers::open_text_file(p.as_path()) {
        eprintln!();
        Err(format!("Could not open configuration file: {}", e))
    } else {
        Ok(())
    }
}

/// Initialize configuration
fn init_config() -> Result<Config, String> {
    let config_dir = path_helpers::init_config_dir()?;
    let Some(config_dir) = config_dir else {
        return Ok(Config::default());
    };

    // if config dir doesn't exist create it
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }

    let config_file = path_helpers::get_config_file(&config_dir)?;
    // Parse configuration
    let config_file =
        file_helpers::open_file_read(config_file.as_path()).map_err(|e| e.to_string())?;
    config_serializer::deserialize(config_file).map_err(|e| e.to_string())
}

/// Get configuration file path
fn get_config_file() -> Option<PathBuf> {
    let config_dir = path_helpers::init_config_dir().ok()??;

    path_helpers::get_config_file(config_dir.as_path()).ok()
}
