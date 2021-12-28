use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Sender<Message>,
}
enum Message {
  NewJob(Job),
  Terminate,
 }
 
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
  fn new(id: usize, receive: Arc<Mutex<Receiver<Message>>>) -> Worker {
    Worker{
      id,
      thread: Some(thread::spawn(move || {
        loop {
          let message = receive.lock().unwrap().recv().unwrap();
          match message {
            Message::NewJob(job) => {
              println!("Worker {} got a job; executing.", id);
              job();
            },
            Message::Terminate => {
              println!("Worker {} while be Terminate", id);
              break;
            },
          }
        }
      }))
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
    self.sender.send(Message::NewJob(job)).unwrap();
  }
 }

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");
    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}
 