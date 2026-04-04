fn main() {
    let x = 2.0;
    let y: f32 = 3.0;

    println!("x: {:.1}, y: {:.1}", x, y);

    let tup: (i32, u32, f64) = (-22, 40, 5.5);
    
    let (x,y,z) = tup;
    println!("The value of x is: {}, y is {} and z is {}", x, y, z);
    let minus_twenty_two = tup.0;
    println!("Value: {}", minus_twenty_two);

}
