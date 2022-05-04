// Copyright (c) 2022 Oliver Wissett
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod scanner;

use crate::scanner::*;
use std::process;

#[macro_use]
extern crate lazy_static;

fn main() {
    // Parse the CLI arguments using clap
    let config = Config::parse();

    // Run the mass scanner
    if let Err(e) = run(config) {
        eprintln!("\x1b[93mApplication Error: {}\x1b[0m", e);
        process::exit(1);
    }
}
