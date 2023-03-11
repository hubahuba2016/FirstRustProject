use std::string;
use std::io;
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
    println!("Store is open at 9AM to 7PM Western Indonesian Time (GMT +7).");
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
        println!("Store is open.");
        println!("WELCOME!");
        let mut is_purchasing = false;
        loop {
            let mut is_purchase_string = String::new();
            println!("Are you coming to purchase? (y/n)");
            io::stdin().read_line(&mut is_purchase_string).expect("Failed to read line");
                if is_purchase_string.to_lowercase().contains("y")
                {
                    println!("How much money do you have?");
                    let mut money_owned = String::new();
                    io::stdin().read_line(&mut money_owned).expect("Failed to read line");
                    let mut money_owned_int:i32 = money_owned.trim().parse().expect("Invalid input");
                    'buyinganything: loop {
                        println!("What do you want to purchase?");
                        println!("1. Cake (50$)");
                        println!("2. Bread (20$)");
                        println!("3. Coffee (15$)");
                        let mut purchase_option = String::new();
                        println!("Select your purchase (1/2/3)");
                        io::stdin().read_line(&mut purchase_option).expect("Failed to read line");
                        let mut purchase_option_int :i32 = purchase_option.trim().parse().expect("Invalid input");
                        let mut total_price = 0;
                        let mut number_of_cakes_str: String;
                        let mut number_of_bread_str: String;
                        let mut number_of_coffee_str: String;
                        let mut number_of_cakes = 0;
                        let mut number_of_bread = 0;
                        let mut number_of_coffee = 0;
                        if purchase_option_int == 1
                        {
                            'buyingcakes: loop {
                                number_of_cakes_str = String::new();
                                println!("How many cakes do you want to buy?");
                                io::stdin().read_line(&mut number_of_cakes_str).expect("Failed to read line");
                                let mut number_of_cakes:i32 = number_of_cakes_str.trim().parse().expect("Invalid input");
                                total_price = number_of_cakes * 50;
                                money_owned_int = money_owned_int - total_price;
                                if money_owned_int < 0
                                {
                                    println!("Sorry, your money is not enough.");
                                    break;
                                } else {
                                    print!("Here you go, you got ");
                                    println!("{} cakes", number_of_cakes);
                                    print!("Here's your change, ");
                                    print!("{}", money_owned_int);
                                    println!("$");
                                    'buyagain: loop
                                    {
                                        let mut buy_again_input = String::new();
                                        println!("Do you want to buy again? (y/n)");
                                        io::stdin().read_line(&mut buy_again_input).expect("Failed to read line");
                                        if buy_again_input.contains("y") || buy_again_input.contains("Y")
                                        {
                                            break 'buyagain;
                                        } else if buy_again_input.contains("n") || buy_again_input.contains("N")
                                        {
                                            break 'buyingcakes;
                                        } else {
                                            println!("I don't understand, let me ask again..");
                                        }
                                    }
                                }
                            }
                        } else if (purchase_option_int == 2)
                        {
                            'buyingbread: loop {
                                number_of_bread_str = String::new();
                                println!("How many breads do you want to buy?");
                                io::stdin().read_line(&mut number_of_bread_str).expect("Failed to read line");
                                let mut number_of_bread:i32 = number_of_bread_str.trim().parse().expect("Invalid input");
                                total_price = number_of_bread * 20;
                                money_owned_int = money_owned_int - total_price;
                                if money_owned_int < 0
                                {
                                    println!("Sorry, your money is not enough.");
                                    break;
                                } else {
                                    print!("Here you go, you got ");
                                    println!("{} breads", number_of_bread);
                                    print!("Here's your change, ");
                                    print!("{}", money_owned_int);
                                    println!("$");
                                    'buyagain: loop
                                    {
                                        let mut buy_again_input = String::new();
                                        println!("Do you want to buy again? (y/n)");
                                        io::stdin().read_line(&mut buy_again_input).expect("Failed to read line");
                                        if buy_again_input.contains("y") || buy_again_input.contains("Y")
                                        {
                                            break 'buyagain;
                                        } else if buy_again_input.contains("n") || buy_again_input.contains("N")
                                        {
                                            break 'buyingbread;
                                        } else {
                                            println!("I don't understand, let me ask again..");
                                        }
                                    }
                                }
                            }
                        } else if purchase_option_int == 3
                        {
                            'buyingcoffee: loop {
                                number_of_coffee_str = String::new();
                                println!("How many coffee do you want to buy?");
                                io::stdin().read_line(&mut number_of_coffee_str).expect("Failed to read line");
                                let mut number_of_coffee:i32 = number_of_coffee_str.trim().parse().expect("Invalid input");
                                total_price = number_of_coffee * 15;
                                money_owned_int = money_owned_int - total_price;
                                if money_owned_int < 0
                                {
                                    println!("Sorry, your money is not enough.");
                                    break;
                                } else {
                                    print!("Here you go, you got ");
                                    println!("{} coffees", number_of_coffee);
                                    print!("Here's your change, ");
                                    print!("{}", money_owned_int);
                                    println!("$");
                                    'buyagain: loop
                                    {
                                        let mut buy_again_input = String::new();
                                        println!("Do you want to buy again? (y/n)");
                                        io::stdin().read_line(&mut buy_again_input).expect("Failed to read line");
                                        if buy_again_input.contains("y") || buy_again_input.contains("Y")
                                        {
                                            break 'buyagain;
                                        } else if buy_again_input.contains("n") || buy_again_input.contains("N")
                                        {
                                            break 'buyingcoffee;
                                        } else {
                                            println!("I don't understand, let me ask again..");
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("Input error, let me ask again..")
                        }
                        println!("Thanks for coming!");
                        /*print!(" you got ");
                        print!("{}", number_of_cakes);
                        print!("cakes,");
                        print!("{}", number_of_bread);
                        print!("breads,");
                        print!("{}", number_of_coffee);
                        println!("coffees");*/
                        print!("And your change is ");
                        print!("{}", money_owned_int);
                        println!("$");
                        break;
                    }
                    break;
                } else if is_purchase_string.to_lowercase().contains("n")
                {
                    println!("Ok! Come again!");
                    break;
                } else {
                    println!("I don't understand your answer, let me ask again..");
                }
            }
    }
    else
    {
        println!("Store is closed.");
    }

}
