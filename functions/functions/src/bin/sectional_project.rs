fn main() {
    apply_to_jobs(5, "Senior Developer");
    let answer = is_even(31);

    println!("Is it an even number {answer}");
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I am applying to {} {} jobs", number, title);
}

fn is_even(number: i32) -> bool {
    let result: i32 = number % 2;

    if result == 0 {
        return true;
    } else {
        return false;
    }
}

fn alphabet(text: &str) -> () {
    let input: &str = text;
    for letters in input {}
}
