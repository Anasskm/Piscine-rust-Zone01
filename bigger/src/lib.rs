use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut big: i32 = 0;
    for (_, value) in h {
        if value > big {
            big = value
        }
    }
    big
}
