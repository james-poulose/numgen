use crate::handlers::utils;
use log::{debug, warn};
use num_bigint::BigUint;

pub fn generate_nnacci_series(dimension: i8, count: Option<u32>) -> Vec<String> {
    const COUNT_DEFAULT: u32 = 10;

    // 10 will be set as default, if not provided..
    let mut count32 = count.unwrap_or(0);
    if count32 <= 0 {
        warn!(
            "No valid value found for argument 'count', defaulting to {}.",
            COUNT_DEFAULT
        );
        count32 = COUNT_DEFAULT;
    }

    debug!("dimension: {}", dimension);
    debug!("count: {}", count32);

    // Create a vector for collecting the nnachi numbers with the base numbers.
    let series: Vec<BigUint> = get_nnacci_series_vec(dimension, count32);

    // Convert BigUint vec to String vec.
    let series_str: Vec<String> = utils::big_uint_to_string_vec(series);

    return series_str;
}

fn get_nnacci_series_vec(dimension: i8, count32: u32) -> Vec<BigUint> {
    let mut series: Vec<BigUint> = create_result_vector(dimension);

    if count32 <= dimension as u32 {
        return series;
    }

    // We already have elements equal to the dimension in the vector, so start the loop from there.
    let mut counter = dimension as u32;

    while counter < count32 {
        let sum = get_sum_of_last_x_digits(&series, dimension);
        debug!("sum: {}", sum);

        series.push(sum);
        counter = counter + 1;
    }

    return series;
}

fn create_result_vector(dimension: i8) -> Vec<BigUint> {
    let mut series: Vec<BigUint> = vec![];

    // For a 2 dimension nacci (Fibonacci), we need 0,1.
    // For a 3 dimension nacci, we need 0,0,1.
    for _ in 0..dimension - 1 {
        series.push(BigUint::from(0u8));
    }

    // Assign the last 1.
    series.push(BigUint::from(1u8));

    return series;
}

fn get_sum_of_last_x_digits(series: &Vec<BigUint>, dimension: i8) -> BigUint {
    let mut sum: BigUint = BigUint::from(0u8);
    let mut counter: i8 = 0;

    for item in series.into_iter().rev() {
        sum = sum + item;
        counter = counter + 1;

        debug!("---------------");
        debug!("Item: {}", item);
        debug!("Counter: {}", counter);

        if counter >= dimension {
            break;
        }
    }

    return sum;
}

#[test]
fn test_nnacci() {
    let mut series = get_nnacci_series_vec(2, 5);
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

    series = get_nnacci_series_vec(3, 5);
    assert_eq!(
        [
            BigUint::from(0u8),
            BigUint::from(0u8),
            BigUint::from(1u8),
            BigUint::from(1u8),
            BigUint::from(2u8)
        ],
        &series[..]
    );

    series = get_nnacci_series_vec(7, 12);
    assert_eq!(
        [
            BigUint::from(0u8),
            BigUint::from(0u8),
            BigUint::from(0u8),
            BigUint::from(0u8),
            BigUint::from(0u8),
            BigUint::from(0u8),
            BigUint::from(1u8),
            BigUint::from(1u8),
            BigUint::from(2u8),
            BigUint::from(4u8),
            BigUint::from(8u8),
            BigUint::from(16u8),
        ],
        &series[..]
    );
}
