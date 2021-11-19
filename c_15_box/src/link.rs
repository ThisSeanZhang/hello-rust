use std::ops::Deref;

#[cfg(test)]
mod tests {
    use crate::link::List::{Cons, Nil};
    use crate::link::{MyBox, hello};

    #[test]
    fn test1() {
        let list = Cons(1,
                        Box::new(
                            Cons(2,
                                 Box::new(
                                     Cons(3, Box::new(Nil))
                                 )
                            )
                        )
        );
    }

    #[test]
    fn diff_point_and_real_num() {

        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y)
    }

    #[test]
    fn test_my_box() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn use_hello_fn() {
        let m = MyBox::new(String::from("Rust"));
        // let a = *m;
        let b = &(*m);
        let b = &((*m)[..]);
        hello(&(*m)[..]);
        hello(&m); // 因为实现了 Deref 可以自动进行解引用转换
    }

}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
