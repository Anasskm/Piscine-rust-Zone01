pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let multiplier = 10f64.powi(2i32);
     (((f - 32.0) * (5.0 / 9.0))*multiplier).round()/multiplier
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    let multiplier = 10f64.powi(2i32);
    (((c * (9.0 / 5.0)) + 32.0)*multiplier).round()/multiplier
}
