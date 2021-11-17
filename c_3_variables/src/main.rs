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
    println!("The const MAX_POINTS is: {}", MAX_POINTS);

    // 隐藏: 重复使用let关键字,隐藏该变量名之前的值
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 可行
    let spaces = " ";
    let spaces = spaces.len();

    println!("space len: {}", spaces);
    // 无法使用mut进行模拟
    // let mut spaces = " ";
    // spaces = spaces.len();

    let int8: i8 = 1;
    let un_signed8: u8 = 1;
    println!("{}{}", int8, un_signed8);

    let x = 2.0; // 双精度
    let y: f32 = 3.0; // 单精度
    println!("{}{}", x, y);

    // 加法
    let sum = 5 + 10;
    // 减法
    let difference = 95.5 - 4.3;
    // 乘法
    let product = 4 * 30;
    // 除法
    let quotient = 56.7 / 32.2;
    // 取余
    let remainder = 43 % 5;
    println!("{}{}{}{}{}", sum, difference, product, quotient, remainder);


    // 单个字节大小
    let t = true;
    let f: bool = false; // 附带了显式类型标注的语句
    println!("{}{}", t, f);

    // char类型占4字节
    let c = 'z';
    let z = ' ';
    let heart_eyed_cat = '🐱';
    println!("{}{}{}", c, z, heart_eyed_cat);

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    let tup = (500, 6.4, 1);
    // 使用解构的方式获取
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {} {} {}", x, y, z);

    // 使用点号(.)来访问
    println!("The value of y is: {} x is : {} z is : {}", tup.0, tup.1, tup.2);

    // 数组类型
    // 动态数组 是指使用Vec创建的数组
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("{:?}{:?}", a, months);
    
    // 定长数组
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{} {} {} {} ", a[0], a[1], a[2], a[3]);
    // 产生的数组等价与  let a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    // 访问数组元素
    println!("{} {} {} {} ", a[0], a[1], a[2], a[3]);


}