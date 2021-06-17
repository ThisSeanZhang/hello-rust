fn main() {
    println!("Hello, world!");

    let v1 = vec![1, 2, 3];
    // 迭代器是惰性的
    let v1_iter = v1.iter();

    // 在for循环中使用迭代器
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // 创建一个新的迭代器
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
