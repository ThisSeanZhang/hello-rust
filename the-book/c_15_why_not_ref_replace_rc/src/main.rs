#[derive(Debug)]
enum List<'a> {
    Cons(i32, &'a List<'a>),
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(10, &Nil);
    println!("{:?}", a);
    let a = Cons(5, &Cons(10, &Nil));
    println!("{:?}", a);
    let b = Cons(3, &a);
    println!("{:?}", b);
    {
        let c = Cons(4, &a);
        println!("{:?}", c);
    }
}
