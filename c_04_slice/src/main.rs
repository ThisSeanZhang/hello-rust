fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // 以下的值都相同
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    println!("slice1 {}, slice2: {}", slice1, slice2);

    let len = s.len();
    let slice1 = &s[3..len];
    let slice2 = &s[3..];
    println!("slice1 {}， slice2： {}", slice1, slice2);

    let len = s.len();
    let slice1 = &s[0..len];
    let slice2 = &s[..];
    println!("slice1 {}， slice2： {}", slice1, slice2);


    let my_string = String::from("hello world");
    // first_word 可以接收String对象的切片作为参数
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word 可以接收字符串字面量的切片作为参数
    let word = first_word(&my_string_literal[..]);
    // 由于字符串字面量本身就是切片，所以我们可以在这里直接将它传入函数，
    // 而不需要使用额外的切片语法！
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
