fn main() {
    println!("Hello, world!");

    let number = 3;
    // 代码中的条件必须产生一个bool类型的值，否则就会触发编译错误
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 使用 else if 进行多重判断
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 如果if 是一个表达式，可以在let语句的右侧使用它来生成一个值
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!(" condition result : {}", number);


     // 但所有的分支产生的值类型要相同 否则会报错
    // let condition = true;
    // let number = if condition {
    //     5
    // } else {
    //     "six"
    // };
    // println!("The value of number is: {}", number);

    // loop循环
    // 下列的代码除了手动终止程序  程序将不会停止
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");


    // 使用for进行集合的遍历

    let mut a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for element in &mut a {
        *element = *element - 5;
        println!("the value is: {}", element);
    }
    println!("after change arr{:?}", a);

    // 使用rev翻转Range产生的序列
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    println!("test {}", gen(4 - 1))
}

fn gen(i: i32) -> i32 {
    if i == 0 { return 0; }
    if i == 1 { return 1; }

    gen(i - 1) + gen(i - 2)
}