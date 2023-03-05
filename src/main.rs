fn main() {
    let myName :String = "Hubaka".to_string();
    let myAge = 25;
    let myAgetoString = myAge.to_string();
    let mut myHelloworld:String = ("Hello, world! My name is ").to_string();
    myHelloworld.push_str(myName.as_str());
    myHelloworld.push_str(", i am ");
    myHelloworld.push_str(myAgetoString.as_str());
    myHelloworld.push_str(" years old!");
    println!("{}",myHelloworld);
    println!("");
    let mut sellingcake :String = ("I am selling a cake.").to_string();
    let mut cakeprice = 30;
    println!("{}", sellingcake);
    print!("The price is ");
    print!("{}",cakeprice);
    println!(" dollars.");
    cakeprice = 50;
    print!("After 5 months, the cake price is ");
    print!("{}", cakeprice);
    print!(" dollars.");


}
