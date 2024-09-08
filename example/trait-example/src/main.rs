use std::{thread::sleep, time::Duration};

const CLEAR : &str = "\x1B[2J\x1B[1;1H";
fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];

    let mut i = 1;

    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        expensive_calculation(n);
    }
}
