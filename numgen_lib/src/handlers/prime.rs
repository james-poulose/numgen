use log::warn;
use num_bigint::BigUint;

use crate::handlers::utils;

pub fn generate_prime_numbers(start: u32, count: Option<u32>) -> Vec<String> {
    const COUNT_DEFAULT: u32 = 5;

    // 10 will be set as default, if not provided.
    let mut count = count.unwrap_or(0);
    if count <= 0 {
        warn!(
            "No valid value found for argument 'count', defaulting to {}.",
            COUNT_DEFAULT
        );
        count = COUNT_DEFAULT;
    }

    let mut series: Vec<BigUint> = vec![];
    let mut maybe_prime = BigUint::from(start);

    while series.len() <= count.try_into().unwrap() {
        if is_prime(&maybe_prime) {
            series.push(maybe_prime.clone());
        }
        maybe_prime = maybe_prime + BigUint::from(1u8);
    }

    // Convert BigUint to String.
    let series_str = utils::big_uint_to_string_vec(series);

    return series_str;
}

fn is_prime(number: &BigUint) -> bool {
    /*
        Iterate through all numbers from 2 to square root of n and for every number check if it divides n [because if a number is expressed as n = xy and
         any of the x or y is greater than the root of n, the other must be less than the root value]. If we find any number that divides, we return false.
    */

    let big_zero = BigUint::from(0u8);
    let big_one = BigUint::from(1u8);

    if number <= &big_one.clone() {
        return false;
    }

    // Check from 2 to sqrt(number).
    let mut divisor = BigUint::from(2u8);

    while divisor <= number.sqrt() {
        let remainder = number % divisor.clone();

        if remainder == big_zero {
            // If one number is fully divisible between 2 & sqrt(n), then this is not a prime, we can exit the loop.
            return false;
        }

        divisor = divisor + big_one.clone();
    }

    return true;
}

#[test]
fn test_prime() {
    // Sub-two numbers.
    assert_eq!(false, is_prime(&BigUint::from(0u8)));
    assert_eq!(false, is_prime(&BigUint::from(1u8)));

    // Super-two numbers (true).
    assert_eq!(true, is_prime(&BigUint::from(197u8)));
    assert_eq!(true, is_prime(&BigUint::from(751u16)));
    assert_eq!(true, is_prime(&BigUint::from(7907u16)));

    // Super-two numbers (false).
    assert_eq!(false, is_prime(&BigUint::from(100u8)));
    assert_eq!(false, is_prime(&BigUint::from(800u16)));
    assert_eq!(false, is_prime(&BigUint::from(8000u16)));
}
