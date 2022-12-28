
use super::KvsEngine;
use crate::{KvsError, Result, thread_pool::ThreadPool};
use log::error;
use sled::Db;
use tokio::sync::oneshot;

/// Wrapper of `sled::Db`
#[derive(Clone)]
pub struct SledKvsEngine<P: ThreadPool> {
    pool: P,
    db: Db
}

impl<P: ThreadPool> SledKvsEngine<P> {
    /// Creates a `SledKvsEngine` from `sled::Db`.
    pub fn open(db: Db, concurrency: u32) -> Result<Self> {
        let pool = P::new(concurrency)?;
        Ok(SledKvsEngine { pool, db })
    }
}

impl<P: ThreadPool> KvsEngine for SledKvsEngine<P> {
    async fn set(&self, key: String, value: String) -> Result<()> {
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();
        self.pool.spawn(move || {
            let res = db
                .insert(key, value.into_bytes())
                .and_then(|_| db.flush())
                .map(|_| ())
                .map_err(KvsError::from);
            if tx.send(res).is_err() {
                error!("Receiving end is dropped");
            }
        });
        match rx.await {
            Ok(v) => v,
            Err(e) => Err(KvsError::from(e))
        }
    }
    async fn get(&self, key: String) -> Result<Option<String>> {
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();
        self.pool.spawn(move || {
            let res = (move || {
                Ok(db
                    .get(key)?
                    .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
                    .map(String::from_utf8)
                    .transpose()?)
            })();
            if tx.send(res).is_err() {
                error!("Receiving end is dropped");
            }
        });
        match rx.await {
            Ok(v) => v,
            Err(e) => Err(KvsError::from(e))
        }
    }

    async fn remove(&self, key: String) -> Result<()> {
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();
        self.pool.spawn(move || {
            let res = (|| {
                db.remove(key)?.ok_or(KvsError::KeyNotFound)?;
                db.flush()?;
                Ok(())
            })();
            if tx.send(res).is_err() {
                error!("Receiving end is dropped");
            }
        });
        match rx.await {
            Ok(v) => v,
            Err(e) => Err(KvsError::from(e))
        }
    }
}