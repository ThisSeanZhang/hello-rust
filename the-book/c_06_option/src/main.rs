fn main() {

    // 常见且实用的Option
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number:Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none)

}
