fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 模式语法
    // 1. 匹配字面量
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 2. 匹配变量名
    // 定义在match表达式内作为模式一部分的变量会覆盖掉match结构外的同名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 3. 多重模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 4. 使用...来匹配值区间
    let x = 5;
    match x {
        1 ... 5 => println!("one through five"),
        6 ..= 7 => println!("6 through 7"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a' ..= 'j' => println!("early ASCII letter"),
        'k' ... 'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 5. 使用结构分析值
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 6. 结构枚举
    let msg = Message::ChangeColor(Color::Rgb(1,2,3));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}",x,y);
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Hsv(r,g,b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        },
        Message::ChangeColor(c) => {
            println!("Change the color to color : {:?}",c);
        }
    }

    // 进行复杂解构
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

}
struct Point {
    x: i32,
    y: i32,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}
