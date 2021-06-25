fn main() {

    /// 函数指针
    // 使用fn类型来接收函数指针作为参数
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);


    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();

    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> =
        (0u32..20)
            .map(Status::Value)
            .collect();

}
