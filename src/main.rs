use chrono::{DateTime, Local, NaiveDateTime, Utc};
use std::time::SystemTime;

fn main() {
    println!("Hello, world!");
    fn age_calculator(year: i32, month: i32, day: i32) -> i32 {
        let current_year = 2024;
        let current_month = 12;
        let current_day = 19;
        let mut age = current_year - year;
        if month > current_month {
            age -= 1;
        } else if month == current_month {
            if day > current_day {
                age -= 1;
            }
        }
        age
    }
    let age = age_calculator(1995, 6, 3);
    println!("Age: {}", age);

    let date = SystemTime::now();
    let now: DateTime<Utc> = date.into();
    println!("{:?}", now);
}
