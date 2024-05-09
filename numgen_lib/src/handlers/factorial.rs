use num_bigint::BigInt;

pub fn compute_factorial(number: i32) -> String {
    // Factorial of zero is 1.
    // https://www.thoughtco.com/why-does-zero-factorial-equal-one-3126598
    if number == 0 {
        return 1.to_string();
    }

    // Factorial of 1, -1, 2 and -2 are 1, -1, 2 and -2 respectively.
    if number == -1 || number == 1 || number == -2 || number == 2 {
        return number.to_string();
    }

    let mut factorial = BigInt::from(1u8);

    for i in 1..=number {
        factorial = factorial * i;
    }

    return factorial.to_string();
}

// Tests
#[test]
fn test_factorial() {
    assert_eq!("1", compute_factorial(0));
    assert_eq!("1", compute_factorial(1));
    assert_eq!("-1", compute_factorial(-1));
    assert_eq!("2", compute_factorial(2));
    assert_eq!("-2", compute_factorial(-2));

    assert_eq!("3628800", compute_factorial(10));
    assert_eq!("93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000", compute_factorial(100));
}
