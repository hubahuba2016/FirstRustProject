fn main() {
    use chrono::prelude::*;
    let my_name:String = "Hubaka".to_string();
    let my_age = 25;
    let my_age_to_string = my_age.to_string();
    let mut my_hello_world:String = ("Hello, world! My name is ").to_string();
    my_hello_world.push_str(my_name.as_str());
    my_hello_world.push_str(", i am ");
    my_hello_world.push_str(my_age_to_string.as_str());
    my_hello_world.push_str(" years old!");
    println!("{}", my_hello_world);
    println!("");
    let mut selling_cake:String = ("I am selling a cake.").to_string();
    let mut cake_price = 30;
    println!("{}", selling_cake);
    print!("The price is ");
    print!("{}", cake_price);
    println!(" dollars.");
    cake_price = 50;
    print!("After 5 months, the cake price is ");
    print!("{}", cake_price);
    println!(" dollars.");
    println!("");
    println!("Store is open at 9AM to 7PM Western Indonesian Time.");
    let now = Utc::now();
    let indonesian_now = now.hour() + 7;
    let mut is_store_open = true;
    if(indonesian_now > 9 && indonesian_now < 19)
    {
        is_store_open = true;
    }
    else
    {
        is_store_open = false;
    }
    if(is_store_open==true)
    {
        println!("Store is open.")
    }
    else
    {
        println!("Store is closed.")
    }

}
