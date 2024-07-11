fn edit_distance(source: &str, target: &str) -> usize {
    let len1 = source.chars().count();
    let len2 = target.chars().count();
    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 0..=len1 {
        for j in 0..=len2 {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else {
                let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                    0
                } else {
                    1
                };
                dp[i][j] = (dp[i - 1][j - 1] + cost)
                    .min(dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1);
            }
        }
    }
    dp[len1][len2]
}
fn is_snake_case(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase() || c == '_')
}
pub fn expected_variable(a: &str, b: &str) -> Option<String> {
    if a.contains(' ') || b.contains(' ') {
        return None;
    }
    let snake_a = a.to_lowercase();
    let snake_b = b.to_lowercase();
    if is_snake_case(&snake_a) && is_snake_case(&snake_b) {
        let distance = edit_distance(&snake_a, &snake_b);
        let similarity = 1.0 - (distance as f64 / b.len() as f64);
        let res = (similarity * 100.0).round();
        if res > 50.0 {
            return Some(format!("{}%", res));
        }
    }
    None
}