use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut res = HashMap::new();
    for word in words {
        *res.entry(word).or_insert(0) += 1
    }
    res
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut count: usize = 0;
    for &value in frequency_count.values() {
        if value == 1 {
            count += 1
        }
    }
    count
}
