mod cli;
mod command_handler;
mod common;
use std::{env::args, time::Instant};

use clap::Parser;
use cli::cli_parser::CliParser;
use log::info;

use crate::common::log_util;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    // CLI argument parsing, error handling, help display are handled in this call. Specific argument related display can be changed in the <command>_args.rs file.
    let cli: CliParser = CliParser::parse();

    // Initialize logging (console & file (if a valid path provided in the args)).
    // Variable cli needs to be borrowed here using '&' or else handle_command will not be able to use it.
    log_util::init_log(args);

    // Start benchmark.
    let start = Instant::now();

    // Pass the cli object (which contains the command and it's arguments) to the generic handler.
    command_handler::handle_command(cli);

    let elapsed_secs = start.elapsed().as_secs_f32();

    // Execution complete.
    println!("");
    info!("Generation completed in {} seconds.", elapsed_secs);
}
