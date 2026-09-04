// Rust calls a drop function at the end of the scope and so heap memory is cleared.
//
// Variable must have a heap allocated data. Drop function in stack does not work.

fn main() {
    let person: String = String::from("Vaibhav Kumar");
    println!("My name is {person}");

    let genius: String = person.clone();
    println!("{person}");
    println!("{genius}")

    // Ownership model - We can Explicit states when a copy should be made.
    //
    // Clone method is a requirement of a triat called cClone.
    // Drop function is use to clear the heap function. It cannot be used to clear the stack memory.
    // It is ok to use copies.
}

// Drop function can be use to clear out heap memory as well as the entry in the stack memory about
// the reference of the heap space.
