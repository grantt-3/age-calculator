fn main() {
    println!("Hello, world!");
    fn age_calculator(year: i32, month: i32, day: i32) -> i32 {
        let current_year = 2021;
        let current_month = 9;
        let current_day = 6;
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
}
