mod handlers;
use handlers::fibonacci::generate_fibonacci_series;

// TODO: We should be able to expose fibonacci.rs (and it's public functions) directly to the cli crate.
// Until that is figured out, keep an entry point in lib.rs.
pub fn generate_fibonacci(start: u64, count: Option<u32>) {
    generate_fibonacci_series(start, count);
}
