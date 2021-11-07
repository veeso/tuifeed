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

use argh::FromArgs;
use std::env;
use std::process::exit;

// includes
mod config;
mod feed;

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
    println!("Hello, world!");
}
