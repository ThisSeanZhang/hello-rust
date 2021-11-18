/*
使用
cargo test --test-threads=1
进行限制测试运行时的线程

 cargo test -- --nocapture
测试运行时打印println!想要打印的内容

cargo test <函数名>
可以给cargo test传递一个测试函数的名称来单独运行该测试

#[ignore]
加上之后将会跳过运行这个测试
cargo test -- --ignored
单独运行那些被忽略的测试
 */
/*
单元测试小而专注，每次只单独测试一个模块或私有接口。而集成测试，
和正常从外部调用代码库一样使用外部代码。只能访问公共，并且在一次测试中
 */
#[cfg(test)]
// 标注上#[cfg(test)]后rust就会在执行cargo test编译执行该部分代码，
// 而在执行cargo build时剔除它们
mod tests {

    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    // 不要在使用Result<T, E> 编写的测试上标注 #[should_panic]
    // 使用这样的返回值就可以在测试函数体中使用问好运算符了
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("Carol"));
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );

    }
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: u32,
}
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }
        Guess {
            value
        }
    }
}


fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
