pub fn to_url(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join("%20")
}