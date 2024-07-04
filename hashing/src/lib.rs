use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    list.iter().sum::<i32>() as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut l = list.clone();
    l.sort();
    let mid = l.len() / 2;
    if l.len() % 2 == 0 {
        return (l[mid - 1] + l[mid]) / 2;
    }
    l[mid]
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut res = HashMap::new();
    for l in list {
        let count = res.entry(l).or_insert(0);
        *count += 1
    }
    let mut prev = 0;
    let mut repeated: i32 = 0;

    for (&key, value) in res {
        if value > prev {
            repeated = key;
            prev = value
        }
    }
    repeated
}
