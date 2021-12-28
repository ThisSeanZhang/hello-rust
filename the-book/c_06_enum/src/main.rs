
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}{:?}", four, six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    println!("{:?}", home);

    println!("{:?}{}", home.kind, home.address);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", loopback);

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    println!("{:?}", home);

    let loopback = IpAddr2::V6(String::from("::1"));
    println!("{:?}", loopback);

    // println!("ip is {}", home)

    let home = IpAddr3::V4(127,0,0,1);
    println!("{:?}", home);

    let loopback = IpAddr3::V6(String::from("::1"));
    println!("{:?}", loopback);

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 以下的结构体和以上枚举每个存储的数据是相同的

    // struct QuitMessage;
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String);
    // struct ChangeColorMessage(i32, i32, i32);

    // 但存在不同地方是我们可以通过定义一个统一处理一个枚举的函数进行处理,而定义多个结构体则不行

    /**
    * 枚举类型也能使用impl进行定义枚举方法
     */

    impl Message {
        fn call(&self) {
            // 此处进行定义方法体
        }

        fn read(&self) {
            match self {
                Message::Quit => println!("Quit"),
                Message::Move{ x , y } => println!("{}, {}", x, y),
                Message::ChangeColor(x, y, z) => println!("{}, {}, {}", x, y, z),
                _=> println!("None")
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    Message::Quit.read();
    Message::Move{ x: 1, y: 2 }.read();
    Message::ChangeColor(1, 2, 3).read();
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String)
}
#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

