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
fn equal_5<T>() -> T
    where T: Fn(u32) -> bool {
    let x:u32 = 5;
    |z| z == x
}