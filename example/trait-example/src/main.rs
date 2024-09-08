use std::{thread::sleep, time::Duration};

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];

    for n in v.iter() {
        expensive_calculation(n);
    }
}
