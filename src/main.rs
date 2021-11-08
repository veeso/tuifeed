/**
 * MIT License
 *
 * tuifeed - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

const TUIFEED_VERSION: &str = env!("CARGO_PKG_VERSION");
const TUIFEED_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

// -- libs
#[macro_use]
extern crate lazy_static;

use argh::FromArgs;
use std::env;
use std::path::PathBuf;
use std::process::exit;

// includes
mod config;
mod feed;
mod helpers;

// -- internal
use config::serializer as config_serializer;
use config::Config;
use helpers::file as file_helpers;
use helpers::open as open_helpers;
use helpers::paths as paths_helpers;

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
            eprintln!("If necessary, you can run tuifeed with `-c` option to edit and eventually fix your configuration file");
            exit(255);
        }
    };
    // TODO: remove and add UI here
    let client = feed::Client::new(config.sources);
    match client.fetch() {
        Err(err) => {
            eprintln!("Failed to get feed: {}", err);
            exit(1);
        }
        Ok(kiosk) => {
            let sources = kiosk.sources();
            for source in sources.into_iter() {
                let feed = kiosk.get_feed(source).unwrap();
                println!("# {} ({})\n", source, feed.title().unwrap_or("?"));
                for article in feed.articles() {
                    if let Some(title) = article.title.as_ref() {
                        println!("## {}\n", &title)
                    }
                    if !article.authors.is_empty() {
                        print!("Written by: ");
                        article.authors.iter().for_each(|x| print!("{} ", x));
                        print!("\n\n");
                    }
                    println!("{}\n", article.summary);
                    println!("Read article at <{}>\n", article.url);
                }
                println!("---\n");
            }
        }
    }
}

/// ### edit_config_file
///
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

/// ### init_config
///
/// Initialize configuration
fn init_config() -> Result<Config, String> {
    let config_dir = paths_helpers::init_config_dir()?;
    if config_dir.is_none() {
        return Ok(Config::default());
    }
    let config_file = paths_helpers::get_config_file(config_dir.unwrap().as_path())?;
    // Parse configuration
    let config_file =
        file_helpers::open_file_read(config_file.as_path()).map_err(|e| e.to_string())?;
    config_serializer::deserialize(config_file).map_err(|e| e.to_string())
}

/// ### get_config_file
///
/// Get configuration file path
fn get_config_file() -> Option<PathBuf> {
    let config_dir = match paths_helpers::init_config_dir() {
        Ok(Some(p)) => p,
        _ => return None,
    };
    match paths_helpers::get_config_file(config_dir.as_path()) {
        Ok(p) => Some(p),
        _ => None,
    }
}
