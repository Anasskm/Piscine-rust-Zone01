use chrono::*;
pub use json::*;
use std::collections::HashMap;
pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    data.members().for_each(|x| {
        let date_string = x["commit"]["author"]["date"].to_string();
        let date =
            chrono::NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%dT%H:%M:%SZ").unwrap();
        let week_number = date.iso_week().week();
        let year = date.year();
        let week_key = format!("{}-W{}", year, week_number);
        let count = map.entry(week_key).or_insert(0);
        *count += 1;
    });
    map
}
pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    //data.members().for_each(|x| println!("{}", x["commit"]["author"]["name"]));
    let mut map = HashMap::new();
    data.members().for_each(|x| {
        let author = x["author"]["login"].to_string();
        let count = map.entry(author).or_insert(0);
        *count += 1;
    });
    map
}
