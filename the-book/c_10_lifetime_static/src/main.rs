use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    println!("get_static(): {}", get_static());
}

fn get_static() -> &'static str {
    let s: &'static str = "I have a static lifetime.";
    s
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}