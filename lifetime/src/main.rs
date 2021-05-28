fn main() {
    // println!("Hello, world!");
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x; //借用x
    //     }// 但到此处时x生命到期了  编译错误
    //     println!("r: {}", r);
    // }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
// 方法无法判断返回哪个引用， 也就无法得知用哪个引用来验证生命周期
// 不过我们也无法判断
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}