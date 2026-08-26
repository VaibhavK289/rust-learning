fn main() {}

// Whether a value is copied and moved to a function parameter will once again depend on the type of
// data it is and whether or not it implements the copy trait.
//
// For stack data like integers, floats and boolean which implements copy trait then Rust will
// create a copy of the data because it follows the copy trait.
//
// For data stored in heap, like string the copy of the data is not created since, it does not
// follow copy trait. Instead a reference to a stack is created.
//
//
