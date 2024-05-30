use log::info;

use crate::cli::{cli_parser::CliParser, commands_type::CommandType};

pub fn handle_command(cli: CliParser) {
    match &cli.command {
        CommandType::Fibonacci(args) => {
            let series = numgen_lib::generate_fibonacci(args.start, args.count);

            let series_str = convert_to_csv(series);
            info!("Fibonacci series for the given input: {}", series_str);
        }
        CommandType::Factorial(args) => {
            let factorial = numgen_lib::compute_factorial(args.number);
            info!("Factorial of {}: {}", args.number, factorial);
        }
        CommandType::Tribonacci(args) => {
            let series = numgen_lib::generate_tribonacci_series(args.count);

            let series_str = convert_to_csv(series);
            info!("Tribonacci series for the given input: {}", series_str);
        }
        CommandType::Prime(args) => {
            let series = numgen_lib::generate_prime_numbers(args.start, args.count);

            let series_str = convert_to_csv(series);
            info!("Prime numbers series for the given input: {}", series_str);
        }
        CommandType::Nnacci(args) => {
            let series = numgen_lib::generate_nnacci_series(args.dimension, args.count);

            let series_str = convert_to_csv(series);
            info!(
                "Nnacci series (dimension: {}) for the given input: {}",
                args.dimension, series_str
            );
        }
    }
}

fn convert_to_csv(array: Vec<String>) -> String {
    // Convert the vector to a comma separated string.
    let mut series_str = array
        .iter()
        .map(|x| x.to_string() + ",")
        .collect::<String>();
    series_str = series_str.trim_end_matches(",").to_string();

    return series_str;
}
