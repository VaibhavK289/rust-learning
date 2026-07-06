fn main() {
    open_store("Mumbai");
    bake_pizza(10, "olive");
    swim_in_profit();
}

fn open_store(neighbourhood: &str) {
    println!("Opening my pizza store in {neighbourhood}");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizza");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}
