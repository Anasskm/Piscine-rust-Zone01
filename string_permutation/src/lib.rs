use std::collections::HashMap;



pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len(){
        return false;
    }
    let mut s1_map = HashMap::new();
    let mut s2_map = HashMap::new();
    for s in s1.chars().into_iter() {
        let count = s1_map.entry(s).or_insert(0);
        *count += 1
    }
    for s in s2.chars().into_iter() {
        let count = s2_map.entry(s).or_insert(0);
        *count += 1
    }
    
    s1_map == s2_map
}