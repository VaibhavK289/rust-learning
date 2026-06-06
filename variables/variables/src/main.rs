/*fn main() {
    let apples: i32 = 50;
    println!("{}", apples);

    let mangoes: i32 = 33;
    println!("{}", mangoes);

    let _fruits: i32 = apples + mangoes;

    println!("Number of apples is {}", apples - 10);
    println!(
        "Number of apples is {1} and mangoes is {0}",
        apples, mangoes
    );
}
*/

// fn main() {
// let gym_reps: i32 = 10;
//println!("I plan to do {gym_reps} reps in gym.");

//let grams_of_protein: &str = "100.345";
//println!("{grams_of_protein}");

//let grams_of_protein: f32 = 100.345;
//println!("{grams_of_protein}");

//let grams_of_protein: u32 = 100;
// println!("{grams_of_protein}");

//{
//    let inner_var: i32 = 22;
//      println!(inner_var);
//  }

//    println!("{}", inner_var);
//}
//

const TAX_RATE: f64 = 7.25;
type Meters = i32;

#[allow(unused_variables)]

fn main() {
    let income: i32 = 10000;
    println!("My income is {income} and my tax rate is {TAX_RATE}");

    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;

    println!(
        "A one mile race is {mile_race_length} meters long and a two mile race is {two_mile_race_length} meters long"
    );

    let total_calories: i32 = 3000;
    let calories_consumed: i32 = 2750;
}
