use crate::{
    cli::{cli_parser::CliParser, commands_type::CommandType},
    handlers::fibonacci::generate_fibonacci_series,
};
pub fn handle_command(cli: CliParser) {
    match &cli.command {
        CommandType::Fibonacci(args) => {
            generate_fibonacci_series(args);
        }
    }
}
