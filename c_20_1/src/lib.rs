use std::thread;
pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
}
impl ThreadPool {
  /// 创建线程池
  /// 
  /// 线程池中的数量
  /// 
  /// # Panics
  /// 当size小于等于0时抛出错误
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);
    let threads = Vec::with_capacity(size);
    
    ThreadPool {
      threads
    }
  }

  pub fn execute<F>(&self, f: F)
  where
  F: FnOnce() + Send + 'static
  {
  }
 }