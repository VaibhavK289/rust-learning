fn main() {
    let my_stack_value: i32 = 2;
    let my_integer_reference: &i32 = &my_stack_value;

    let my_heap_value: String = String::from("Toyota");
    let my_heap_reference: &String = &my_heap_value;

    // Dereference means to access the data at the memory address that the reference points to.
    println!("{}", *my_integer_reference);
    println!("{}", my_integer_reference);
}
