// Ownership is a set of compiler rules -- Ensures there are no memory bugs
//
// In c++ programmer is responsible for allocating and deallocating memory
// Common errors in manual memory management is - 1. Forget to dellocate memory 2. Deallocating
// already deallocated memory.
//
// Garbage collector is a program that runs parallel to our main program - it dellocates memory that
// is not longer in use. Disadvantages - It can run at disadvantegous times and can slow down
// execution of main program.
//
// Best of both worlds - speed and memory safety although it is a new paradigm and takes time to
// master it.
//
//Program will not be compiled if the ownership principle is violated.
//
//Owner - Entity responsible for that resource cleaning up a piece of data.

fn main() {
    println!("Hello, world!");
}
