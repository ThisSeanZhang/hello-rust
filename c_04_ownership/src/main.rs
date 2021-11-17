fn main() {
    {
        let s = String::from("hello");
    
        takes_ownership(s);
        // 当不能自动复制的变量传入函数后，将会转变所有权，即使是调用同样的方法，也会报错
        // takes_ownership(s);
        let x = 5;
        makes_copy(x);
        // 因为i32是Copy的,所以这个调用之后依旧可以使用
    }
    // x首先离开作用域
    // 因为s的值已经发生了移动,所以此处不会进行操作
    
    {
        let s1 = give_ownership();
        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);
        // s2 传入 又被归还
    }
    // s3先进行销毁
    // s2因为移动过所以不做操作
    // s1销毁

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}// 离开作用域,没有什么特别的事情发生

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}// drop函数被自动调用, some_string所占的内存被自动释放


fn takes_and_gives_back(a_string: String) -> String {
    a_string
}// 作为返回值移至调用处

fn give_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}// 作为返回值移至调用处

/**
* 将一个值赋值给另一个变量时就会转移所有权,当一个持有 **堆数据** 的变量离开作用域时,它的数据就会被drop清理回收,除非这些数据的所有权已经移动
*/

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()会返回当前字符串的长度
    (s, length)
}// 返回多个数据的所有权