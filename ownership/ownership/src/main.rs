// Ownership is a set of compiler rules -- Ensures there are no memory bugs
//
// In c++ programmer is responsible for allocating and deallocating memory
// Common errors in manual memory management is - 1. Forget to dellocate memory 2. Deallocating
// already deallocated memory.
//
// Garbage collector is a program that runs parallel to our main program - it dellocates memory that
// is not longer in use. Disadvantages - It can run at disadvantegous times and can slow down
// execution of main program.

fn main() {
    println!("Hello, world!");
}
