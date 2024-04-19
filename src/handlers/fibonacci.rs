use log::{info, warn};

use crate::cli::fibonacci_args::FibonacciArgs;

pub fn generate_fibonacci_series(args: &FibonacciArgs) {
    let _start = args.start;
    const COUNT_DEFAULT: u32 = 10;

    // 10 will be set as default, if not provided..
    let mut count = args.count.unwrap_or(0);
    if count <= 0 {
        warn!(
            "No valid value found for argument 'count', defaulting to {}.",
            COUNT_DEFAULT
        );
        count = COUNT_DEFAULT;
    }

    let series = get_series_vec(count);

    // Convert the vector to a comma separated string.
    let mut series_str = series
        .iter()
        .map(|x| x.to_string() + ",")
        .collect::<String>();
    series_str = series_str.trim_end_matches(",").to_string();
    info!("Fibonacci series for the given input: {}", series_str);
}

fn get_series_vec(count: u32) -> Vec<u64> {
    // Add the first zero.
    let mut series = vec![0];

    let mut current = 1;
    let mut last = 0;

    for _n in 1..count {
        let next = last + current;
        series.push(next);
        last = current;
        current = next;
    }

    return series;
}
/* fn isPerfectNum(number: f64) -> bool {
    let r = number.sqrt();

    return false;
} */

#[test]
fn test_basic() {
    let series = get_series_vec(5);
    assert_eq!([0, 1, 2, 3, 5], &series[..]);
}
