use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    let number1: u32 = input1.trim().parse().expect("Please enter a valid number");
    let number2: u32 = input2.trim().parse().expect("Please enter a valid number");

    let sum = number1 + number2;
    let difference = number1 - number2;
    let product = number1 * number2;
    let quotient = number1 / number2;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}
