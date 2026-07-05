fn main() {
    // A generic is a placeholder for an abstract type.
    // Generic is a placeholder for a future generic type

    let month_days: std::ops::Range<i32> = 1..31;

    for month_day in month_days {
        println!("{month_day}");
    }
}
