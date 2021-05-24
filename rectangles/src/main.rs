fn main() {
    println!("Hello, world!");
    let weight1 = 30;
    let height1 = 50;

    println!("area is {}", area(weight1, height1));
}
fn area(weight:u32, height:u32) -> u32 {
    weight * height
}