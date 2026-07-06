fn main() {
    let number: i32 = 1_337;
    let second_number: i16 = number as i16;

    let floating_number: f64 = 3.1415926535897932385;

    println!("{} {}", number, second_number);
    println!("Value to pi upto 3 digit precision {:.3}", floating_number);

    let with_milk: bool = true;
    let with_sugar: bool = false;

    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    println!("{}", is_my_type_of_coffee);

    let is_acceptable_coffee: bool = with_milk || with_sugar;
    println!("{}", is_acceptable_coffee);

    let prime_numbers: [i8; 4] = [13, 17, 19, 23];

    dbg!(prime_numbers);

    let tuple_declaration: (i32, f64, bool) = (number, floating_number, is_my_type_of_coffee);
    dbg!(tuple_declaration);
}
