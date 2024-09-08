use std::{collections::HashSet, thread::sleep, time::Duration};

const CLEAR : &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter:Iter,
    i: usize,
    bound: Option<usize>,
    delims: (char, char)
}

impl <Iter> Progress<Iter> {
    pub fn new(iter:Iter) -> Self {
        Progress { iter, i: 0, bound: None, delims: ('[', ']') }
    }
}

impl <Iter> Progress<Iter>
where Iter: ExactSizeIterator {

    pub fn with_bound(mut self) -> Self{
        self.bound = Some(self.iter.len());
        self
    }
}

impl <Iter> Progress<Iter> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self{
        self.delims = delims;
        self
    }
}

impl <Iter> Iterator for Progress<Iter>
where Iter: Iterator{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        match self.bound {
            Some(bound) => println!("{}{}{}{}", self.delims.0, "*".repeat(self.i), " ".repeat(bound - self.i), self.delims.1),
            None => println!("{}", "*".repeat(self.i))
        }
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

/// for **all type** <Iter> impl ProgressIteratorExt
impl<Iter> ProgressIteratorExt for Iter
where Iter: Iterator {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let brkts = ('<', '>');
    // with_delims not works, but not hit
    // key is: Progress is stateful, and with_delims can be call  all status
    for n in (0 ..).progress().with_delims(brkts) {
        expensive_calculation(&n);
    }

    let v = vec![1, 2, 3];

    for n in v.iter().progress().with_bound().with_delims(brkts) {
        expensive_calculation(n);
    }

}
