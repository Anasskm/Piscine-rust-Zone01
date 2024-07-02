pub fn rev_str(input: &str) -> String {
    let char_v: Vec<char> = input.chars().collect();
    let mut res = String::new();
    for i in (0..char_v.len()).rev() {
        res.push(char_v[i])
    }

    res
}
