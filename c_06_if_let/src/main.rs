fn main() {
    println!("Hello, world!");
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 上下代码等价,使用if let能更简便的判断需要的特殊情况
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // 如果被比较的类没有实现PartialEq那么 == 将不能工作
    // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    if Some(3) == some_u8_value {
        println!("three");
    }

}
