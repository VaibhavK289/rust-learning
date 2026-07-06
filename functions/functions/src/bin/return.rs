fn main() {
    let result: i32 = square(33);
    println!("{result}");

    let result: i32 = square(15);
    println!("{result}");

    let result: () = mystery();
}

fn square(number: i32) -> i32 {
    number * number
}

fn mystery() {
    println!("Hello there");
}

// In rust there are two styles in which one can return a value of function.
// First is to use 'return keyword before the main returning value of the function and ending the
// statement with a semi-colon as a end to a change of thought.
// Second is to remove 'return' keyword and semi-colon and leave it like that. Rust compiler
// interprets it as incomplete chain of thought and presumes that it will be continued somewhere
// else which is exactly what we want to do. To complete code into the main function.
//
// A unit is an empty tuple without values
