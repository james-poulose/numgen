use num_bigint::BigUint;

pub fn big_uint_to_string_vec(series: Vec<BigUint>) -> Vec<String> {
    let mut series_str: Vec<String> = vec![];

    for x in series.iter() {
        series_str.push(x.to_string());
    }

    return series_str;
}
