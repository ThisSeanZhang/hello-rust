#![deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result};
pub use engines::{KvStore, SledKvsEngine, KvsEngine};
pub use client::KvsClient;
pub use server::KvsServer;

mod error;
mod command;
mod engines;
mod client;
mod protocol;
mod server;
/// thread_pool
pub mod thread_pool;
