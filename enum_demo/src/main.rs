use std::borrow::Borrow;

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr2::V6(String::from("::1"));

    // println!("ip is {}", home)

    let home = IpAddr3::V4(127,0,0,1);

    let loopback = IpAddr3::V6(String::from("::1"));


    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 以下的结构体和以上枚举每个存储的数据是相同的

    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);

    // 但存在不同地方是我们可以通过定义一个统一处理一个枚举的函数进行处理,而定义多个结构体则不行

    /**
    * 枚举类型也能使用impl进行定义枚举方法
     */

    impl Message {
        fn call(&self) {
            // 此处进行定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    /**
    * 常见且实用的Option
    */

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number:Option<i32> = None;

}

fn route(ip_type:IpAddrKind) {

}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
enum IpAddrKind {
    V4,
    V6,
}
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String)
}
enum IpAddr2 {
    V4(String),
    V6(String),
}

