use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    // 使用move使线程获得所有权
    let handle = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}