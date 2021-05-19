fn main() {
    let s1 = give_ownership();
    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    // s2 传入 又被归还

}
// s3先进行销毁
// s2因为移动过所以不做操作
// s1销毁

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