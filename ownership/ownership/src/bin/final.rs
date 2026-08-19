fn main() {}

// Ownership complete
//
// String - A dynamic piece of text stored on the heap at runtime.
//
// &String ("ref String") - A reference to a heap String.
//
// Important - Ownership model is enforced by compiler. It applied to the variables storing values
// in the heap. But if the data is stored in the heap and other functions using referencing then it
// is stores its memeory address. This way it can use value of other variable without constraints of
// ownership.
