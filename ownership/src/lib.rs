pub fn first_subword(mut s: String) -> String {
    // Convert the string to lowercase for easier processing


    // Find the index of the first lowercase letter or underscore
    let index = match s.chars().skip(1).position(|c| c.is_uppercase() || c == '_') {
        Some(idx) => idx+1,
        None => return s, // Return the original string if no sub-word is found
    };

    // Truncate the string up to the found index
    s.truncate(index);
    s
}