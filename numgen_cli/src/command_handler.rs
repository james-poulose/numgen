use log::info;

use crate::cli::{cli_parser::CliParser, commands_type::CommandType};

pub fn handle_command(cli: CliParser) {
    match &cli.command {
        CommandType::Fibonacci(args) => {
            let series = numgen_lib::generate_fibonacci(args.start, args.count);

            // // Convert the vector to a comma separated string.
            let mut series_str = series
                .iter()
                .map(|x| x.to_string() + ",")
                .collect::<String>();
            series_str = series_str.trim_end_matches(",").to_string();
            info!("Fibonacci series for the given input: {}", series_str);
        }
    }
}
