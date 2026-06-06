#![allow(unused_variables)]

const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "winter";
    println!("{}", season);

    let mut points_scored: i32 = 28;
    println!("{}", points_scored);

    points_scored = 35;
    println!("{}", points_scored);

    let event_time: &str = "06.00";
    let event_time: i32 = 6;

    println!("{0},{1},{2},", points_scored, event_time, TOUCHDOWN_POINTS);

    let favorite_beverage: &str = "coca cola";
}
