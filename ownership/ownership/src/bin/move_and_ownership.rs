// move is transfer of ownership from one owner to another.
// Rust does not create a duplicate of original value of heap data. Instead what is does is it
// copies the stack entry everytime we assign a value of one variable to another.
//
// Since heap is for expandable data like string. A entry is more memory expensive hence to minimize
// it rust creates a copy of its reference instead of another copy in the heap memory. It makes
// utilisation of heap much more effective.
fn main() {
    let person: String = String::from("Vaibhav Kumar");
    let genius: String = person;
}
