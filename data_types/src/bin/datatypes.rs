/*fn main() {
    println!("Hello World");

    let var: i64 = 444443;

    let points_scored: i32 = 200;
    let _days: usize = 55;
    let _years: isize = -15_000;

    println!("{points_scored}");

    println!("{var}");

    println!("{var}");
    println!("Dear Emily, \n How are you ?");
    println!("Ankit said \"I went to stadium\"");

    let filepath: &str = r"C:\My Document\new\videos";
    println!("{filepath}");

    let home: &str = "C:\\My Drive\\Functions";
    println!("{home}");
}
*/

fn main() {
    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_space: &str = "          my content          ";
    println!("{}", empty_space);
    println!("{}", empty_space.trim());

    let pi: f64 = 3.14159265358979323822;
    println!("{pi}");
}
