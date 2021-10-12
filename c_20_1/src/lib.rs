use std::thread;
pub struct ThreadPool {
  workers: Vec<Worker>,
}

struct Worker {
  id: usize,
  handle: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize) -> Worker {
    Worker{
      id,
      handle: thread::spawn(|| {})
    }
  }
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
    let mut workers = Vec::with_capacity(size);
    
    for id in 0..size {
      workers.push(Worker::new(id));
    }

    ThreadPool {
      workers
    }
  }

  pub fn execute<F>(&self, f: F)
  where
  F: FnOnce() + Send + 'static
  {
  }
 }