use crate::cli::{cli_parser::CliParser, commands_type::CommandType};

pub fn handle_command(cli: CliParser) {
    match &cli.command {
        CommandType::Fibonacci(args) => {
            numgen_lib::generate_fibonacci(args.start, args.count);
        }
    }
}
