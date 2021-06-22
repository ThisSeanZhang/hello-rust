use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        // 需要使用v但是线程运行时长不确定, 是不能明确v的引用是否一直有效
        println!("Here's a vector: {:?}", v);
    });
    drop(v); // 例如可能在运行中出现变量被释放，前面的引用就失效了
    handle.join().unwrap();
}