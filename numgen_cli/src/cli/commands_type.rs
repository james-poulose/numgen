use clap::Subcommand;

use super::factorial_args::FactorialArgs;
use super::fibonacci_args::FibonacciArgs;

#[derive(Subcommand)]
pub enum CommandType {
    /// Generates Fibonacci series.
    Fibonacci(FibonacciArgs),

    /// Generates the Factorial of a given number.
    Factorial(FactorialArgs),
}
