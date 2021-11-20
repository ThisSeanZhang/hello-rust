use std::slice;

fn main() {
    // 通过引用创建裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 创建一个指向任意内存地址的裸指针
    let address = 0x012345usize;
    let r = address as *const i32;

    // 在unsafe块中解引用裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }


    // 不安全的函数或方法
    //
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }



    // 使用extern函数调用外部代码
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 定义可被C调用的代码
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }


}

// 以下代码不能运行
// Rust的借用检查器无法理解我们正在借用一个切片的不同部分
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    (&mut slice[..mid],
     &mut slice[mid..])
}
// Rust的借用检查器无法理解我们正在借用一个切片的不同部分，
// 它只知道我们借用了两次同一个切片。借用一个切片的不同部分从原
// 理上来讲应该是没有任何问题的，因为两个切片并没有交叉的地方，
// 但Rust并没有足够智能到理解这些信息。当我们能够确定某段代码的
// 正确性而Rust却不能时，不安全代码就可以登场了
fn split_at_mut2(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
// 基于任意内存地址创建一个切片
// 这段代码试图用一个随意的内存地址来创建拥有10 000个元素的切片 可能导致崩溃
fn error_demo() {
    let address = 0x01234usize;
    let r = address as *mut i32;
    let slice : &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
}


