pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.chars().count();
    let n = target.chars().count();

    // Create a 2D vector to store the edit distances
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize the first row and column
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // Compute edit distance using dynamic programming
    for i in 1..=m {
        for j in 1..=n {
            let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };

            dp[i][j] = (dp[i - 1][j] + 1) // Deletion
                .min(dp[i][j - 1] + 1)   // Insertion
                .min(dp[i - 1][j - 1] + cost); // Substitution
        }
    }

    // Return the edit distance between the source and target strings
    dp[m][n]
}