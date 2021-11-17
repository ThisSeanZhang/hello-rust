use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;
use std::error::Error;

fn main() {
    println!("Hello, world!");
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    // 越界访问数组也会抛错
    // v[99];

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem:{:?}", e),
    //         },
    //         other_error => panic!("There was a problem opening the file: {:?}", other_error),
    //     },
    // };

    // 以下的写法也是等价的
    // let f = File::open("hello.txt").map_err(|error| {
    //     // if error.kind() == ErrorKind::NotFound {
    //     if ErrorKind::NotFound = error.kind() {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:?}", error);
    //         })
    //     } else {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // });

    // 失败时快速触发panic的方法 使用unwrap
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    fn read_username_from_file() -> Result<String, io::Error> {
        // let f = File::open("hello.txt");
        // let mut f = match f {
        //   Ok(file) => file,
        //   Err(e) => return Err(e),
        // };
        // let mut s = String::new();
        // match f.read_to_string(&mut s) {
        //   Ok(_) => Ok(s),
        //   Err(e) => Err(e),
        // }

        // 使用?运算符将错误返回给调用者函数

        // 使用?运算符
        // 错误将会作为方法的返回值,如同使用了return
        // 成功将会作为这个表达式的结果
        // 使用了？运算符的函数必须返回Result、Option或任何实现了std::ops::Try的类型
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    if let Ok(s) = read_username_from_file() {
        println!("{}", s);
    }
    if let Ok(s) = read_username_from_file2() {
        println!("{}", s);
    }
}
/**
* 还可以通过链式方法进行简化
 */
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)

    // 单纯的缩短
    // fs::read_to_string("hello.txt")
}


// fn main2() -> () {
//     let f = File::open("hello.txt")?;
// }
/*
这里的Box<dyn Error>被称作trait对象，我们将在第17章讨论
它。现在，你可以简单地将Box<dyn Error>理解为“任何可能的错
误类型”。在拥有这种返回类型的main函数中使用？运算符是合法
的
 */
// fn main3() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(())
// }
