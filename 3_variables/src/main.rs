fn main() {
    // 默认情况下rust的变量不可变
    // 使用mut关键字能使关键字能重新赋值
    // 但是常量的声明还是使用count进行指定
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 常量的定义
    const MAX_POINTS: u32 = 100_000;

    // 隐藏

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 可行
    let spaces = " ";
    let spaces = spaces.len();

    println!("space len: {}", spaces)
    // 错误
    // let mut spaces = " ";
    // spaces = spaces.len();
}
