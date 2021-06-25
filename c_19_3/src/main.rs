use std::ops::Add;
use std::fmt;

fn main() {
    struct Counter{}
    /// 关联类型 指定 占位类型
    pub trait Iterator {
        type Item;// 占位类型
        fn next(&mut self) -> Option<Self::Item>;
    }

    // 使用具体的类型
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            // --略--
            Option::from(3 as u32)
        }
    }


    /// 默认泛型参数和运算符重载
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    // trait Add<RHS=Self> {
    //     type Output;
    //     fn add(self, rhs: RHS) -> Self::Output;
    // }


    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });

    struct Millimeters(u32);
    struct Meters(u32);
    // 为Millimeters实现Add trait，从而使Millimeters和Meters可以相加
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    /// 用于消除歧义的完全限定语法：调用相同名称的方法
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // 当无self时
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    /// 有时，你会需要在一个trait中使用另外一个trait的功能。在这种
    /// 情况下，我们需要使当前trait的功能依赖于另外一个同时被实现的
    /// trait。这个被依赖的trait也就是当前trait的超trait（supertrait）

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}

    Point{x:1, y: 2}.outline_print();

    /// 使用newtype模式在外部类型上实现外部trait

    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"),
                         String::from("world")]);
    println!("w = {}", w);

}
