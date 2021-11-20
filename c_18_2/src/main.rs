
fn main() {

    // 使用 _ 忽略整个值
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    // 忽略值的某些部分
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // 忽略多个特定值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    // 使用 _ 开头的变量,进行规避未使用变量警告
    let _x = 5; // 以下画线开始的变量名可以避免触发变量未使用警告
    let y = 10;

    //使用以下画线开头的变量名与仅仅使用_作为变量名存在一个细微的差别：
    // _x语法仍然将值绑定到了变量上，而_则完全不会进行绑定
    let s = Some(TestDrop{str: String::from("test")});
    if let Some(_s) = s { // 此处会将s内字符的所有权移到_s
        println!("匹配内部");
    }// 且在此处被释放
    println!("匹配外部");
    // println!("{:?}", s); // 抛错

    let s = Some(TestDrop{str: String::from("test")});
    if let Some(_) = s {
        println!("匹配内部");
    }
    println!("匹配外部: {:?}", s); // 不抛错

    /// 使用..忽略剩余部分
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x:X1, .. } => println!("x is {}", X1),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
    // match numbers {
    //     (.., second, ..) => { // 产生歧义 抛错
    //         println!("Some numbers: {}", second)
    //     },
    // }

    // 在模式上添加一个匹配守卫
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    /// @绑定
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }


}

#[derive(Debug)]
struct TestDrop{
    str: String
}

impl Drop for TestDrop {

    fn drop(&mut self) {
        println!("清理TestDrop");
    }
}