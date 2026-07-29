fn main() {
    // This is called a string literal. It is not stored in either stack or heap but rather it is
    // directly embedded into the rust binary.
    let food: &str = "pasta";

    // String type is completely different.
    // We need dynamic string type whose size we can change at runtime.
    //
    // Mainly used for mutation operations hence stored in the heap.
    //
    println!("Hello");

    let text: String = String::new();

    // Push string method to mutate the string.
    // Push string method is not workable for string slice or string literal.
}
