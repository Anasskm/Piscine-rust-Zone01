use chrono::{Datelike, NaiveDate, Weekday};

fn middle_day(year: i32) -> Option<Weekday> {
    let start = NaiveDate::from_ymd_opt(year, 1, 1)?;
    let end = NaiveDate::from_ymd_opt(year + 1, 1, 1)?;
    let days = end.signed_duration_since(start).num_days() as u32;
    if days % 2 == 0 {
        return None;
    }
    let z =  NaiveDate::from_yo_opt(year, (days / 2)+1)?;
    Some(z.weekday())
}