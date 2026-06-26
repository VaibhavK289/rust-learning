fn main() {
    println!("{}", "1" == "2");
    println!("{}", "coke" == "pepsi");
    println!("{}", "coke" != "pepsi");
    println!("{}", "Coke" == "Coke ");

    let purchase_ticket: bool = false;
    let plane_on_time: bool = false;
    let making_event: bool = purchase_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected", making_event);
}
