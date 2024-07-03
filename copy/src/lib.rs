pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_value = (c as f64).exp();
    let log_value = (c.abs() as f64).ln();

    (c, exp_value, log_value)
}

pub fn str_function(a: String) -> (String, String) {
    // Split the string into individual parts
    let parts: Vec<&str> = a.split_whitespace().collect();

    // Calculate the exponential function for each part and collect into a new Vec
    let exp_values: Vec<String> = parts
        .iter()
        .map(|&part| {
            let value: f64 = part.parse().unwrap(); // Parse the string into a float
            format!("{}", value.exp()) // Calculate the exponential and convert to string
        })
        .collect();

    // Join the calculated exponential values into a single string
    let exp_string = exp_values.join(" ");

    // Return the original string and the exponential string as a tuple
    (a, exp_string)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let res = b
        .iter()
        .map(|&r| {
            let value = (r.abs() as f64).ln();
            value
        })
        .collect();
    (b, res)
}
