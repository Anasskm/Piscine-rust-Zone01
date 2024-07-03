pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_digit(10))
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0)
    });
    words
        .iter()
        .map(|&word| {
            let removed = word.chars().filter(|c| c.is_alphabetic()).collect();
            removed
        })
        .collect::<Vec<String>>()
        .join(" ")
}