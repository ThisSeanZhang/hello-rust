fn main() {
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

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}// 离开作用域,没有什么特别的事情发生

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}// drop函数被自动调用, some_string所占的内存被自动释放
