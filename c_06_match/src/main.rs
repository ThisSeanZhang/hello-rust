fn main() {
    println!("Hello, world!");
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState:: Alaska));
    value_in_cents(Coin::Quarter(UsState:: Alabama));

    // 用于匹配其他值(可以用具体的单词进行命名)
    let some_u8_value = 4u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // 使用_匹配未能匹配的情况
    }

}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u32 {
     match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter (state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}