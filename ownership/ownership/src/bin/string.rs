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

    // When we create a string using String::from() syntax we create a string in the heap memory but
    // it also creates an string entry in the stack. The stack entry contains 3 information -
    // Reference, Length, Capacity. Reference - address, Length - bytes used to store the string.
    // For english characters 1 letter = 1 byte. Capacity - storage available in the heap memory.
    // Generally capacity is greater than length because string is mutable can its size could
    // increase or decrease and if length becomes greater than capacity memory allocator finds a new
    // location for string in the heap memory.

    let mut name: String = String::from("Vaibhav");
    // Although strings are mutable by nature but we still has to use the mut keyword to notify
    // compiler that we are going to do mutability operations on the string.

    println!("{name}");

    name.push_str(" Kumar Kandhway");
    println!("{name}");

    //push_str method can be used to push new characters in the existing string.
}
