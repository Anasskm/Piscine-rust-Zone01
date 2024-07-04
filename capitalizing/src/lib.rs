pub fn capitalize_first(input: &str) -> String {
    if let Some(first) = input.chars().next() {
        let c = first.to_ascii_uppercase().to_string();
        let r = input
            .chars()
            .skip(1)
            .collect::<String>()
            .to_ascii_lowercase();
        return format!("{}{}", c, r);
    }
    String::new()
}
pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|s| capitalize_first(s))
        .collect::<Vec<String>>()
        .join(" ")
}
pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|s| {
            if s.is_ascii_uppercase() {
                s.to_ascii_lowercase()
            } else {
                s.to_ascii_uppercase()
            }
        })
        .collect()
}