fn main() {
    let seasons: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);

    println!("{:?}", seasons);

    dbg!(seasons);

    // do not leave any debug macros in the main production code because it is for benefit of
    // developers and not intended to go in production.
}
