// This code teaches one how can we use simple blocks(or scopes) as a functions with limited
// utilities.
// Key features - It contains two curly braces to define the scope. It also contains a implicit type
// return statement declaration.
// We can assign the block to a variable. This helps in printing the values that is inside the scope
// outside the scope as it essentially acts as a function block instead of a regular scope
// declaration.
//
// Drawbacks - It does not have a complete set of features that a function provide. Instead of
// calling it a function-alike it would be much more accurate to call it a block expressions.
// It lacks certain features such as parameters, reusability and early returns.
//
// Since, Rust Compiler compute assignment operator from right to left, it first computes the contents of the
// nested block and then it stores the return value in calculation variable and value variable is
// out of scope and cannot be used.

fn main() {
    let multiplier: i32 = 3;

    let calculation = {
        let value: i32 = 5 + 4;
        value * multiplier
    };

    println!("{calculation}");
}
