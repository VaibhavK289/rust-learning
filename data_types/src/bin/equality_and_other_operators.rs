fn main() {
    println!("{}", "1" == "2");
    println!("{}", "coke" == "pepsi");
    println!("{}", "coke" != "pepsi");
    println!("{}", "Coke" == "Coke ");

    let purchase_ticket: bool = false;
    let plane_on_time: bool = false;
    let making_event: bool = purchase_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected", making_event);

    let user_has_paid_for_subscription: bool = true;
    let user_is_admin: bool = true;
    let user_can_see_premium_experience: bool = user_has_paid_for_subscription || user_is_admin;
    println!("Can this user see my site? {user_can_see_premium_experience}");
}
