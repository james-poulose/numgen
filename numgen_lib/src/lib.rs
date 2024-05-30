mod handlers;

// TODO: We should be able to expose all files in "handlers" folder (e.g. fibonacci.rs) (and it's public functions) directly to the cli crate.
// Until that is figured out, keep an entry point in lib.rs.

pub fn generate_fibonacci(start: u64, count: Option<u32>) -> Vec<String> {
    return handlers::fibonacci::generate_fibonacci_series(start, count);
}

pub fn compute_factorial(number: i32) -> String {
    return handlers::factorial::compute_factorial(number);
}

pub fn generate_tribonacci_series(count: u32) -> Vec<String> {
    return handlers::tribonacci::generate_tribonacci_series(count);
}
pub fn generate_prime_numbers(start: u32, count: Option<u32>) -> Vec<String> {
    return handlers::prime::generate_prime_numbers(start, count);
}

pub fn generate_nnacci_series(dimension: i8, count: Option<u32>) -> Vec<String> {
    return handlers::nnachi::generate_nnacci_series(dimension, count);
}
