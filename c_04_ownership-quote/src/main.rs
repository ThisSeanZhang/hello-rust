fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 在不转移所有权的情况下，创建一个s1值的引用
    println!("The length of '{}' is {}.", s1, len);

    // 使用可变的引用需要以下条件
    // 1. 变量声明成mut类型
    let mut s2 = String::from("hello");
    // 2. 参数传入时传入一个可变引用
    change(&mut s2);

    // 但对于特定作用域的特定数据来说可变引用一次只能声明一个
    // 以下将会抛出 cannot borrow `s3` as mutable more than once at a time
    // let mut s3 = String::from("hello");
    // let r1 = &mut s3;
    // let r2 = &mut s3;
    //
    // println!("{}, {}", r1, r2)

    let mut s4 = String::from("hello");
    {
        let r11 = &mut s4;

    } // 由于 r11 在这里离开了作用域，所以我们可以合法地再创建一个可变引用。
    let r12 = &mut s4;


    // 不可变引用可以出现多次,但不能和可变引用混用
    // cannot borrow `s5` as mutable because it is also borrowed as immutable
    // let mut s5 = String::from("hello");
    // let r21 = &s5;
    // let r22 = &s5;
    // let r23 = & mut s5;
    // println!("{}, {}, {}", r21, r22, r23)

    // 垂直指针
    // 指向了曾经存在某处内存地址
    let reference_to_nothing = dangle();

}
fn calculate_length(s: &String) -> usize {
    s.len()
    // 引用的值不可以进行改变
    // s.push_str(", world");
}// 离开作用域时,因为不持有s的所有权,所以不会释放s

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 返回包含了一个借用,但是要被借用的值已经不存在了
fn dangle() -> &String {
    let s = String::from("hello");
    &s // 返回s的借用
} // 因为此处就要将s进行销毁了

/**
* 任意一段给定的时间,你要么只能拥有一个可变引用,要么只能拥有任意数量的不可变引用
* 引用总是有效的
*/