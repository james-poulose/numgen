use clap::Subcommand;

use super::factorial_args::FactorialArgs;
use super::fibonacci_args::FibonacciArgs;
use super::tribonacci_args::TribonacciArgs;

#[derive(Subcommand)]
pub enum CommandType {
    /// Generates Fibonacci series.
    Fibonacci(FibonacciArgs),

    /// Computes the Factorial of a given number.
    Factorial(FactorialArgs),

    /// Generates Tribonacci series.
    Tribonacci(TribonacciArgs),
}
