fn main() {}

// Whether a value is copied and moved to a function parameter will once again depend on the type of
// data it is and whether or not it implements the copy trait.
//
// For stack data like integers, floats and boolean which implements copy trait then Rust will
// create a copy of the data
