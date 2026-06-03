fn main() {
    let apples: i32 = 50;
    println!("{}", apples);

    let mangoes: i32 = 33;
    println!("{}", mangoes);

    let _fruits: i32 = apples + mangoes;

    println!("Number of apples is {}", apples - 10);
    println!(
        "Number of apples is {1} and mangoes is {0}",
        apples, mangoes
    );
}
