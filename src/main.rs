#![feature(with_options)]

mod cli;
mod colors;
mod constants;
mod context;
mod store;

use cli::Cli;
use std::env;
use store::create_config_dir;

fn main() {
    create_config_dir();

    let mut cli = Cli::new();

    let arguments: Vec<String> = env::args().collect();

    // the first argument is the executable path, it can be ignored
    cli.execute_command(&arguments[1..]);
}
