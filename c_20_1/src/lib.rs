use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Sender<Job>,
}

struct Worker {
  id: usize,
  handle: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
  fn new(id: usize, receive: Arc<Mutex<Receiver<Job>>>) -> Worker {
    Worker{
      id,
      handle: thread::spawn(move || {
        loop {
          let job = receive.lock().unwrap().recv().unwrap();
          println!("Worker {} got a job; executing.", id);
          job();
        }
      })
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
    let (sender, receiver) = mpsc::channel();
    let share_receiver = Arc::new(Mutex::new(receiver));
    for id in 0..size {
      workers.push(Worker::new(id, share_receiver.clone()));
    }

    ThreadPool {
      workers,
      sender
    }
  }

  pub fn execute<F>(&self, f: F)
  where
  F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    self.sender.send(job).unwrap();
  }
 }