#[derive(Debug)] // 加上debug的注解
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 这种传入&self叫做方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}
// 可以有多个impl块
impl Rectangle {
    // 这种不传入&self叫做函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}