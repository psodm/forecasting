use chrono::{Local, NaiveDate, Datelike, Weekday};

pub fn get_first_day_of_current_week() -> NaiveDate {
    let today = Local::now().naive_local().date();
    let weekday = today.weekday();

    // Calculate the number of days to subtract to get to Monday
    // Monday is 0, Tuesday is 1, ..., Sunday is 6
    let days_to_subtract = match weekday {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    today - chrono::Duration::days(days_to_subtract as i64)
}