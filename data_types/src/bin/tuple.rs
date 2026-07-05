use std::ops::Range;
use std::ops::RangeInclusive;

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

    let month_days: Range<i32> = 1..31;
    println!("{month_days:?}");

    let month_days: RangeInclusive<i32> = 1..=31;
    println!("{month_days:?}");

    for number in month_days {
        println!("{number}");
    }

    let letters = 'b'..'f';
    for letter in letters {
        println!("{letter}");
    }

    let colours = ["Red", "Green", "Yellow"];

    for colour in colours {
        println!("{colour} is a great colour!");
    }
}
