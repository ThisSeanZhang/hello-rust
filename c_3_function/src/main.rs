fn main() {
    println!("Hello, world!");

    // 调用其他方法
    another_function(5);


    // 在Rust中存在语句和表达式的概念
    // 语句指的是: 执行操作 但是没有返回值的操作
    // 表达式则是: 会进行计算并产生一个值作为结果的指令

    // 语句
    let y = 6;
    println!("{}", y);
    // 而以下是错误的  因为语句没有返回值 所以不能进行赋值
    // let x = (let y = 6);

    // 而以下的操作可以进行  因为返回的是一个表达式
    // 因为结尾处的 "x + 1" 没有带上分号 表示为这个表达式的返回
    // 如果我们加上了分号 那么这个表达式就没有返回  将会抛错
    let x = 5;
    println!("{}", plus_one(x));
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);


    let x = five();
    println!("The value of x is : {}", x);

}


// 声明方法
// 参数使用形参和类型的键值对进行表示
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

// 表示函数的返回值类型是i32
fn five() -> i32 {
    5
}

// 如果将下列的函数返回语句后加上分号 那么将会导致表达式变为语句导致错误
fn plus_one(x: i32) -> i32 {
    x + 1
}


