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
  let rect2 = &Rectangle::square(5);
  println!("The area of the rectangle is {} square pixels.", rect1.area());
  println!("{}{}", rect1.height, rect1.width);
  println!("can_hold: {}", rect1.can_hold(rect2));
  println!("can_hold: {}", Rectangle::can_hold(rect2,&rect1));
}