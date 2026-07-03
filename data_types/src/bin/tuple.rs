fn main() {
    let employee: (&str, i32, &str) = ("Molly", 32, "Marketing");

    //let name: &str = employee.0;
    //let age: i32 = employee.1;
    //let department: &str = employee.2;
    //
    let (name, age, department) = employee;
    println!("Name: {name}, age: {age}, department: {department}");

    println!("{employee:#?}");

    dbg!(employee);
}
