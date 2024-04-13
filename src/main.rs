mod cli;
mod command_handler;
use clap::Parser;
use cli::cli::Cli;

use crate::command_handler::handle_command;

fn main() {
    let cli: Cli = Cli::parse();
    handle_command(cli);
    println!("Hello, world!");
}
