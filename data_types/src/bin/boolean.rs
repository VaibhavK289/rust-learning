fn main() {
    /*
    let is_handsome: bool = true;
    let is_silly: bool = false;

    println!("{is_handsome} and Silly {is_silly}");

    let age: i32 = 40;
    let is_young: bool = age < 35;
    println!("{is_young}");
    println!("{} {}", age.is_positive(), age.is_negative());

    println!("{}", !true);
    print!("{}", !false);
    */

    let age = 13;
    let can_see_rated_r_movie: bool = age >= 17;
    let cannot_see_rated_r_movie: bool = !can_see_rated_r_movie;

    println!("I am {age} years old. Can I see this scary movie? {can_see_rated_r_movie}");
}
