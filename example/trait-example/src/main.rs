use std::{collections::HashSet, thread::sleep, time::Duration};

const CLEAR : &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter:Iter,
    i: usize,
}

impl <Iter> Progress<Iter> {
    pub fn new(iter:Iter) -> Self {
        Progress { iter, i: 0 }
    }
}

impl <Iter> Iterator for Progress<Iter>
where Iter: Iterator{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

/// for **all type** <Iter> impl ProgressIteratorExt
impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {

    // you can write this
    let x = 1.progress();
    let y = "blah".progress();
    // but can not iter it
    // for _ in x {}

    let v = vec![1, 2, 3];

    for n in v.iter().progress() {
        expensive_calculation(n);
    }

}
