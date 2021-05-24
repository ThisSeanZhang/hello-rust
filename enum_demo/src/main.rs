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

}

fn route(ip_type:IpAddrKind) {

}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrKind {
    V4,
    V6
}

