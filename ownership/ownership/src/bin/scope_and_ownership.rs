fn main() {
    let age: i32 = 33;
    // Variable is the owner of a value. In this case variable 'age' is the owner of the value 33.

    {
        let is_handsome: bool = true;
    } // is handsome goes out of scope here.
    // age variable exists here.
} //age variable goes out of scope here.
//
//
