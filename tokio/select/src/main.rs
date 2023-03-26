
use std::pin::Pin;
use std::time::{Instant, Duration};

use futures::future::{Fuse, FusedFuture, FutureExt};
use futures::select;

// 定义一个异步函数，它会延迟3秒后返回"Hello"
async fn say_hello() -> &'static str {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    "Hello"
}

// 定义一个异步函数，它会使用select!宏等待say_hello或者5秒后触发
async fn wait_or_timeout() {
    // 创建一个延迟5秒的Sleep Future，并调用fuse方法
    let mut timeout = Box::pin(tokio::time::sleep(std::time::Duration::from_secs(5)).fuse());
    // 创建一个say_hello Future，并调用fuse方法
    let mut hello = Box::pin(say_hello().fuse());
    let time = Instant::now();
    // 使用select!宏同时等待timeout和hello
    loop {
      select! {
        // 如果timeout完成了，打印超时信息
        () = timeout => {
            println!("Timeout");
        }
        // 如果hello完成了，打印它的返回值
        result = hello => {
            println!("Result: {}", result);
            timeout = Box::pin(tokio::time::sleep(std::time::Duration::from_secs(5)).fuse());
            hello = Box::pin(say_hello().fuse());
            if time.elapsed() > Duration::from_secs(20) {
                break;
            }
        }
      };
    }
}

// 在tokio运行时中执行wait_or_timeout函数
#[tokio::main]
async fn main() {
    wait_or_timeout().await;
}