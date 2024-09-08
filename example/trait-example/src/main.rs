use std::{collections::HashSet, thread::sleep, time::Duration};

const CLEAR : &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter:Iter,
    i: usize,
    bound: Option<usize>
}

impl <Iter> Progress<Iter> {
    pub fn new(iter:Iter) -> Self {
        Progress { iter, i: 0, bound: None }
    }
}

impl <Iter> Progress<Iter>
where Iter: ExactSizeIterator {

    pub fn with_bound(mut self) -> Self{
        self.bound = Some(self.iter.len());
        self
    }
}

impl <Iter> Iterator for Progress<Iter>
where Iter: Iterator{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        match self.bound {
            Some(bound) => println!("[{}{}]", "*".repeat(self.i), " ".repeat(bound - self.i)),
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

    // The method can be used, but a compilation error will be prompted: "ExactSizeIterator  was not satisfied"
    // for n in (0 ..).progress().with_bound() {
    //     expensive_calculation(&n);
    // }

    let v = vec![1, 2, 3];

    for n in v.iter().progress().with_bound() {
        expensive_calculation(n);
    }

}
