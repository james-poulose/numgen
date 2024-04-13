use crate::cli::cli::Cli;
use crate::cli::commands_type::CommandType;
use crate::executors::calculate;
pub fn handle_command(cli: Cli) {
    match &cli.command {
        CommandType::Fibonacci(_args) => {
            println!("handling...");
            calculate();
        }
    }
}
