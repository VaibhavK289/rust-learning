// Rust calls a drop function at the end of the scope and so heap memory is cleared.
//
// Variable must have a heap allocated data. Drop function in stack does not work.

fn main() {
    let person: String = String::from("Vaibhav Kumar");
    println!("My name is {person}");

    drop(person);
    print!("{person}");

    // Ownership model - We can Explicit states when a copy should be made.
    //
    // Clone method is a requirement of a triat called cClone.
    // Drop function is use to clear the heap function. It cannot be used to clear the stack memory.
    //
}
