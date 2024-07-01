pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    round_to_n_decimal_places ( (f - 32.0) * (5.0 / 9.0),2)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
   round_to_n_decimal_places ((c * (9.0 / 5.0)) + 32.0, 2)
}

fn round_to_n_decimal_places(value: f64, n: u32) -> f64 {
    let multiplier = 10f64.powi(n as i32);
    (value * multiplier).round() / multiplier
}