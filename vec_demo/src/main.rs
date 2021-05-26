fn main() {
    println!("Hello, world!");
    let v:Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // 这样访问时 越界方法直接触发panic
    println!("The third element is {}", third);
    match v.get(3) { // 使用get访问 越界会获得None
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    if let Some(get) = v.get(4) {
        println!("in if let The 4 element is {}", get)
    }

    // 不能在同一个作用域中同事获取可变引用和不可变引用
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);


    // 遍历数组元素
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 解引用运算符（*）来获得i绑定的值
        println!("{}", i);
    }

    // 在动态数组中使用定义的枚举来存储不同类型的值
    // 如果无法列举出所有可能的情况话,还可以使用trait进行定义
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // String也是一种字符串
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    // 这个方法同样也可以直接作用于字面量：
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // 传入的是字符切片  为了传入后  后续还能继续使用
    println!("s2 is {}", s2);

    // 也可传入单个字符
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意这里的s1已经被移动且再也不能被使用了
    // println!("s1: {}", s1);
    /**
    * 由于+ 的函数签名是fn add(self, s: &str) -> String
    * self不是取引用,所以s1的所有权被转移,而s2使用的是&String传入了引用
    * 将会被强转成&str 变为&s2[..]
    * 即便let s3 = s1 + &s2;看起来像是复制两个字符串并创建一个新的字符串，但实际上这
    * 条语句会取得s1的所有权，再将s2中的内容复制到其中，最后再将s1的所有权作为结果返回
    **/

    // 如果遇到需要拼接多个字符串的

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    // format 不会转移所有权

    // Rust的字符串不支持索引
    // let s1 = String::from("hello");
    // let h = s1[0];

    // 因为String的内部是使用u8进行 且使用utf-8进行编码
    let len = String::from("Hola").len();// 会返回4
    // 但下方的长度就不是12了 实际是24 因为每个字符需要占用2个字节
    let len = String::from("Здравствуйте").len();
    println!("&[0..2]: {}", &"Здравствуйте"[0..2]);
    // 以下的方式进行获取会报错,因为不是取到一个完整的字符, 所以不建议使用切片的方式获取字符
    // thread 'main' panicked at 'byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`', src\main.rs:110:30
    // println!("&[0..3]: {}", &"Здравствуйте"[0..3]);
    // 再如 印度的梵文 9 个字符  但实际上占用了27个字节
    let len = String::from("संस्कृतम्").len();
    println!("len {}", len);

    // 遍历字符
    for c in "संस्कृतम्".chars() {
        // println!("{}", c);
    }
    // 合法的Unicode标量值可能会需要占用1字节以上的空间
    for b in "संस्कृतम्".bytes() {
        // println!("{}", b);
    }

}
// 与其他struct一样  动态数组离开作用域 将会被立即销毁