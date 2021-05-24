#[derive(Debug)] // 加上debug的注解
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1); // 加上了debug的注解才能进行格式化输出
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
 fn area(rectangle: &Rectangle) -> u32 {
     rectangle.width * rectangle.height
}