use log::{debug, info, warn};
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

    let series = get_fib_series_vec(start, count);

    // Convert BigUint to String.
    let mut series_str: Vec<String> = vec![];

    for x in series.iter() {
        series_str.push(x.to_string());
    }

    return series_str;
}

fn get_fib_series_vec(start: u64, count: u32) -> Vec<BigUint> {
    let big_zero = BigUint::from(0u8);
    let big_one = BigUint::from(1u8);
    let mut series: Vec<BigUint>;

    debug!("count: {}", count);
    // You will either get a 0 as the start number (because Fibonacci series always starts with a zero), or a Fibonacci
    // number (if you are starting from a random number supplied by --start).
    let mut last: BigUint = get_start_num(&BigUint::from(start));
    debug!("last: {}", last);

    // Calculate the second number in the series (whether starting from 0 or not).
    let mut current: BigUint = get_next_fibonacci(&last);
    debug!("current: {}", current);

    // Initialize the result array with the start values depending where it starts from.

    if last == big_zero {
        series = vec![big_zero, big_one];
    } else {
        series = vec![last.clone(), current.clone()];
    }

    for _ in 1..count - 1 {
        let next = &last + &current;
        series.push(next.clone());
        last = current;
        current = next;
    }

    return series;
}

fn get_start_num(start: &BigUint) -> BigUint {
    let big_zero = BigUint::from(0u8);
    if start <= &big_zero {
        return big_zero;
    };

    // Get the next highest fibonacci
    let next = get_next_fibonacci(start);
    return next;
}

fn get_next_fibonacci(number: &BigUint) -> BigUint {
    // Start an infinite loop until we find the next fibonacci.
    // TODO: Risk analysis - Extreme large numbers, O(n).
    let mut num = number.clone();
    loop {
        // We don't care if the current number is a fibonacci. We want to find the next fibonacci in the number line.
        num = num + BigUint::from(1u8);

        let is_fibonacci = is_fibonacci(&num);

        if is_fibonacci {
            return num;
        }
    }
}

fn is_fibonacci(number: &BigUint) -> bool {
    //  If we take any number x, it will be a Fibonacci number if and only if (5x^2)+4 or (5x^2)-4 is a perfect square.
    // https://www.baeldung.com/kotlin/fibonacci-number-test#using-perfect-square-property
    let part1 = BigUint::from(5u8) * number * number;
    info!("number: {}", &number);
    info!("part1: {}", &part1);
    let four_bi: BigUint = BigUint::from(4u8); // 4.to_biguint().unwrap();
    info!("four_bi: {}", &four_bi);
    let maybe_perfect_sq_1 = &part1 + &four_bi;
    let maybe_perfect_sq_2 = &part1 - &four_bi;
    let result = is_perfect_square(&maybe_perfect_sq_1) || is_perfect_square(&maybe_perfect_sq_2);

    return result;
}

fn is_perfect_square(number: &BigUint) -> bool {
    let big_int = number.sqrt();

    return &big_int * &big_int == *number;
}

#[test]
fn test_basic() {
    let mut series = get_fib_series_vec(0, 5);
    assert_eq!(
        [
            BigUint::from(0u8),
            BigUint::from(1u8),
            BigUint::from(1u8),
            BigUint::from(2u8),
            BigUint::from(3u8)
        ],
        &series[..]
    );

    series = get_fib_series_vec(6, 3);
    assert_eq!(
        [BigUint::from(8u8), BigUint::from(13u8), BigUint::from(21u8)],
        &series[..]
    );

    series = get_fib_series_vec(6760, 3);
    assert_eq!(
        [
            BigUint::from(6765u16),
            BigUint::from(10946u16),
            BigUint::from(17711u16)
        ],
        &series[..]
    );
}

#[test]
fn test_perfect_square() {
    assert_eq!(true, is_perfect_square(&BigUint::from(64u8)));
    assert_eq!(false, is_perfect_square(&BigUint::from(7u8)));
}

#[test]
fn test_get_next_fibonacci() {
    // Test with non-fibonacci inputs.
    assert_eq!(
        BigUint::from(55u8),
        get_next_fibonacci(&BigUint::from(50u8))
    );
    assert_eq!(
        BigUint::from(165580141u32),
        get_next_fibonacci(&BigUint::from(165570141u32))
    );

    // Test with fibonacci inputs.
    assert_eq!(BigUint::from(2u8), get_next_fibonacci(&BigUint::from(1u8)));
    assert_eq!(BigUint::from(3u8), get_next_fibonacci(&BigUint::from(2u8)));
    assert_eq!(
        BigUint::from(6765u16),
        get_next_fibonacci(&BigUint::from(4181u16))
    );
}
