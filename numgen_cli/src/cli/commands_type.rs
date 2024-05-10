use clap::Subcommand;

#[derive(Subcommand)]
pub enum CommandType {
    /// Generates Fibonacci series.
    Fibonacci(super::fibonacci_args::FibonacciArgs),

    /// Computes the Factorial of a given number.
    Factorial(super::factorial_args::FactorialArgs),

    /// Generates Tribonacci series.
    Tribonacci(super::tribonacci_args::TribonacciArgs),

    /// Generates the Prime numbers
    Prime(super::prime_args::PrimeArgs),
}
