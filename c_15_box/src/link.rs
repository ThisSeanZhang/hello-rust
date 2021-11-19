use std::ops::Deref;

#[cfg(test)]
mod tests {
    use std::ops::Deref;

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
        let c = *y; // 因为基础类型可以进行复制
        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(x, *y);
    }

    fn diff_point_and_real_str() {

        let x = String::from("Hello");
        let y = &x;
        // let c = *y; // 与 diff_point_and_real_num 测试不同,因为String 不能复制
        assert_eq!(String::from("Hello"), x);
        assert_eq!(String::from("Hello"), *y);
        assert_eq!(x, *y);
    }

    #[test]
    fn test_my_box() {
        let x = 5;
        let y = MyBox::new(x);
        let c = *y;
        let d = *(y.deref()); // 上一行会被隐式的转为下面这样的形式
        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(x, *y);
    }

    /*
    Rust会在类型与trait满足下面3种情形时执行解引用转换
    * 当T: Deref<Target=U>时，允许&T转换为&U
    * 当T: DerefMut<Target=U>时，允许&mut T转换为&mut U
    * 当T: Deref<Target=U>时，允许&mut T转换为&U
    */
    #[test]
    fn use_hello_fn() {
        let m = MyBox::new(String::from("Rust"));
        // let a = *m; String 没有实现Copy 所以不能复制
        let b = &(*m);
        // let b = (*m)[..]; str不能被赋值
        let b = &((*m)[..]);
        let c = &m;
        let c: &str = &m;
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
