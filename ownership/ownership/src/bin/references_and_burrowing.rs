// Burrowing means using something without taking ownership of it.
//Reference is an address in computer's memorywhere the original value is stored.
fn main() {
    let my_stack_value: i32 = 2;
    let my_integer_reference: &i32 = &my_stack_value;

    let my_heap_value: String = String::from("Toyota");
    let my_heap_reference: &String = &my_heap_value;
}
