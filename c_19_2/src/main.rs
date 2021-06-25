static HELLO_WORLD: &str = "Hello, world!";
fn main() {
    println!("name is: {}", HELLO_WORLD);
    // 可变静态变量的读写都是不安全的
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

/// 使用unsafe trait
unsafe trait Foo {
    // 某些方法
}

unsafe impl Foo for i32 {
    // 对应的方法实现
}
