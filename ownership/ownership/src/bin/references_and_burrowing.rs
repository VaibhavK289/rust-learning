// Burrowing means using something without taking ownership of it.
//Reference is an address in computer's memorywhere the original value is stored.
fn main() {
    let my_stack_value: i32 = 2;
    let my_integer_reference: &i32 = &my_stack_value;

    let my_heap_value: String = String::from("Toyota");
    let my_heap_reference: &String = &my_heap_value;\

    let my_stack_value: i32 = 2;
    let my_integer_reference: &i32 = &my_stack_value;
}

// Instead of storing values again and again in heap causing repeating value we create an entry in
// heap and multiple entries containing address of it in stack memory. So that its value could be
// retrieved even without changing ownership. Reference is cheaper to store. Reference can hold a
// value of either stack and heap.
