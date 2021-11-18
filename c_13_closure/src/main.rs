use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
    let six: u32 = 6;
    let compare= equal_5();
    println!("eq {}", compare(six))
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}


fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {

    // 一般闭包会进行自动推导 可以不进行推导
    // let expensive_closure = |num: u32| -> u32 {
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });


    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("take a break")
        } else {
            println!(
                "today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }

}

fn test() {
    // 虽然闭包的类型可以进行推导，但是不能进行两次不同类型的调用
    // 以下进行编译时会出现错误
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

// 使用闭包的特性
// 为什么不能直接使用Fn(u32) -> bool 作为返回
// https://stackoverflow.com/questions/67945027/return-closures-but-cannot-infer-type
fn equal_5() -> impl Fn(u32) -> bool {
    let x:u32 = 5;
    move |z| z == x
}

// FnOnce意味着闭包可以从它的封闭作用域中，也就是闭包所处的环境中，消耗捕获的变量。
// 为了实现这一功能，闭包必须在定义 时取得这些变量的所有权并将它们移动至闭包中。
// 这也是名称FnOnce中Once一词的含义：因为闭包不能多次获取并消耗掉同一变量的所有权，所以它只能被调用一次。
// FnMut可以从环境中可变地借用值并对它们进行修改。
// Fn可以从环境中不可变地借用值。