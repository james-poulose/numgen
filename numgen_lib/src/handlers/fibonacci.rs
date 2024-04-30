use core::str;

use log::{debug, warn};
use num_bigint::BigUint;

pub fn generate_fibonacci_series(start: u64, count: Option<u32>) -> Vec<String> {
    const COUNT_DEFAULT: u32 = 10;

    // 10 will be set as default, if not provided..
    let mut count = count.unwrap_or(0);
    if count <= 0 {
        warn!(
            "No valid value found for argument 'count', defaulting to {}.",
            COUNT_DEFAULT
        );
        count = COUNT_DEFAULT;
    }

    let series = get_series_vec_x(start, count);

    // Convert BigUint to String.
    let mut series_str: Vec<String> = vec![];

    for x in series.iter() {
        series_str.push(x.to_string());
    }

    return series_str;
}

fn get_series_vec(start: u64, count: u32) -> Vec<u64> {
    let mut series;

    // You will either get a 0 as the start number (because Fibonacci series always starts with a zero), or a Fibonacci
    // number (if you are starting from a random number supplied by --start).
    let mut last = get_start_num(start);

    // Calculate the second number in the series (whether starting from 0 or not).
    let mut current = get_next_fibonacci(last);

    debug!("last: {}", last);
    debug!("current: {}", current);
    debug!("count: {}", count);

    // Initialize the result array with the start values depending where it starts from.
    if last == 0 {
        series = vec![0, 1];
    } else {
        series = vec![last, current];
    }

    for _n in 1..count - 1 {
        let next = last + current;
        series.push(next);
        last = current;
        current = next;
    }

    return series;
}

fn get_series_vec_x(start: u64, count: u32) -> Vec<BigUint> {
    let big_zero = BigUint::from(0u8);
    let big_one = BigUint::from(1u8);
    let mut series: Vec<BigUint>;

    debug!("count: {}", count);
    // You will either get a 0 as the start number (because Fibonacci series always starts with a zero), or a Fibonacci
    // number (if you are starting from a random number supplied by --start).
    let mut last: BigUint = get_start_num_x(&BigUint::from(start));
    debug!("last: {}", last);

    // Calculate the second number in the series (whether starting from 0 or not).
    let mut current: BigUint = get_next_fibonacci_x(&last);
    debug!("current: {}", current);

    // Initialize the result array with the start values depending where it starts from.

    if last == big_zero {
        series = vec![big_zero, big_one];
    } else {
        series = vec![last.clone(), current.clone()];
    }

    for _ in 1..count - 1 {
        let next = (&last + &current);
        series.push(next.clone());
        last = current;
        current = next;
    }

    return series;
}

fn get_start_num(start: u64) -> u64 {
    if start <= 0 {
        return 0;
    };

    // Get the next highest fibonacci
    let next = get_next_fibonacci(start);
    return next;
}

fn get_start_num_x(start: &BigUint) -> BigUint {
    let big_zero = BigUint::from(0u8);
    if start <= &big_zero {
        return big_zero;
    };

    // Get the next highest fibonacci
    let next = get_next_fibonacci_x(start);
    return next;
}

fn get_next_fibonacci(mut number: u64) -> u64 {
    // Start an infinite loop until we find the next fibonacci.
    // TODO: Risk analysis - Extreme large numbers, O(n).
    loop {
        // We don't care if the current number is a fibonacci. We want to find the next fibonacci in the number line.
        number += 1;
        let is_fibonacci = is_fibonacci(number);

        if is_fibonacci {
            return number;
        }
    }
}

fn get_next_fibonacci_x(number: &BigUint) -> BigUint {
    // Start an infinite loop until we find the next fibonacci.
    // TODO: Risk analysis - Extreme large numbers, O(n).
    let mut num = number.clone();
    loop {
        // We don't care if the current number is a fibonacci. We want to find the next fibonacci in the number line.
        num = num + BigUint::from(1u8);

        let is_fibonacci = is_fibonacci_x(&number);

        if is_fibonacci {
            return num;
        }
    }
}

fn is_fibonacci(number: u64) -> bool {
    //  If we take any number x, it will be a Fibonacci number if and only if (5x^2)+4 or (5x^2)-4 is a perfect square.
    // https://www.baeldung.com/kotlin/fibonacci-number-test#using-perfect-square-property
    let result = is_perfect_square((5 * number * number) + 4)
        || is_perfect_square((5 * number * number) - 4);

    return result;
}

fn is_fibonacci_x(number: &BigUint) -> bool {
    //  If we take any number x, it will be a Fibonacci number if and only if (5x^2)+4 or (5x^2)-4 is a perfect square.
    // https://www.baeldung.com/kotlin/fibonacci-number-test#using-perfect-square-property
    let part1 = BigUint::from(5u8) * number * number;
    let four_bi: BigUint = BigUint::from(4u8); // 4.to_biguint().unwrap();
    let maybe_perfect_sq_1 = &part1 + &four_bi;
    let maybe_perfect_sq_2 = &part1 - &four_bi;
    let result =
        is_perfect_square_x(&maybe_perfect_sq_1) || is_perfect_square_x(&maybe_perfect_sq_2);

    return result;
}

fn is_perfect_square(number: u64) -> bool {
    let sqrt = (number as f64).sqrt().round() as u64;

    return sqrt * sqrt == number;
}

fn is_perfect_square_x(number: &BigUint) -> bool {
    let big_int = number.sqrt();
    //let sqrt = (number as f64).sqrt().round() as u64;

    return big_int.sqrt() == *number;
}

#[test]
fn test_basic() {
    let mut series = get_series_vec(0, 5);
    assert_eq!([0, 1, 1, 2, 3], &series[..]);

    series = get_series_vec(6, 3);
    assert_eq!([8, 13, 21], &series[..]);

    series = get_series_vec(6760, 3);
    assert_eq!([6765, 10946, 17711], &series[..]);
}

#[test]
fn test_perfect_square() {
    assert_eq!(true, is_perfect_square(64));
    assert_eq!(false, is_perfect_square(7));
}

#[test]
fn test_get_next_fibonacci() {
    // Test with non-fibonacci inputs.
    assert_eq!(55, get_next_fibonacci(50));
    assert_eq!(165580141, get_next_fibonacci(165570141));

    // Test with fibonacci inputs.
    assert_eq!(2, get_next_fibonacci(1));
    assert_eq!(3, get_next_fibonacci(2));
    assert_eq!(6765, get_next_fibonacci(4181));
}
