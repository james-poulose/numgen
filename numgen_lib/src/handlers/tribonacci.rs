use num_bigint::BigUint;

pub fn generate_tribonacci_series(count: u32) -> Vec<String> {
    let mut first = BigUint::from(0u8);
    let mut second = BigUint::from(1u8);
    let mut third = BigUint::from(1u8);
    let mut series: Vec<BigUint> = vec![first.clone(), second.clone(), third.clone()];

    for _ in 1..count - 1 {
        let next = &first + &second + &third;
        series.push(next.clone());

        first = second;
        second = third;
        third = next;
    }

    // Convert BigUint to String.
    let mut series_str: Vec<String> = vec![];

    for x in series.iter() {
        series_str.push(x.to_string());
    }

    return series_str;
}
