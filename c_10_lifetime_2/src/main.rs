fn main() {
    println!("Hello, world!");
}

/*
指定生命周期的方式往往取决于函数的具体功能。打个比方，假
如将longest函数的实现修改为返回第一个而不是最长的那个字符串
切片参数，那么我们就无须再为y参数指定生命周期。下面的代码是
可以通过编译的
*/
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// 垂直指针
// 无法通过编译
// 因为返回值的生命周期没有与如何参数的生命周期产生关联
// 且结束时result将被清理 引用是无效的
fn longest2<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
