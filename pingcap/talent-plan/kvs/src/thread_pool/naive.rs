use std::thread;

use super::ThreadPool;

/// NaiveThreadPool
pub struct NaiveThreadPool {

}

impl ThreadPool for NaiveThreadPool {
  fn new(_threads: u32) -> crate::Result<Self> {
    Ok(NaiveThreadPool{})
  }

  fn spawn<F>(&self, job: F)
  where
    F: FnOnce() + Send + 'static
  {
    thread::spawn(job);
  }
}