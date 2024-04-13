use clap::Subcommand;

use super::fibonacci_args::FibonacciArgs;

#[derive(Subcommand)]
pub enum CommandType {
    /// Generates Fibonacci series.
    Fibonacci(FibonacciArgs),
}
