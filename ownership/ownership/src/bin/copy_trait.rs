//In this case since the value of both time and year is 2025 different copies of value 2025 would be
//stored in the stack separatly.

// The value is last to the stack would be removed first so in this case year value would be removed
// first.

fn main() {
    let time: i32 = 2025;
    let year: i32 = time;

    println!("The time is {time}. It is the year {year}");

    let ice_cream: &str = "COokies and Cream";
    let dessert: &str = ice_cream;

    println!("{ice_cream}");
}

// If we have a variable storing a reference and we assign that to another variable. Rust will
// create a copy of the reference.ffinal.rs
//
// Notes
