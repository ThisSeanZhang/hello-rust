use std::{
    collections::BTreeMap,
    path::{PathBuf, Path},
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write, Seek, SeekFrom, self, Read, BufReader}, ffi::OsStr, sync::{Arc, atomic::{AtomicU64, Ordering}, Mutex}, cell::RefCell, future::Future,
};

use crossbeam::queue::ArrayQueue;
use crossbeam_skiplist::SkipMap;
use futures::TryFutureExt;
use log::error;
use serde_json::Deserializer;
use tokio::sync::oneshot;

use crate::{error::Result, KvsError, KvsEngine, thread_pool::ThreadPool};
use crate::command::{Command, CommandPos};

fn log_path(path: &Path, gen: impl ToString) -> PathBuf {
    path.join(format!("{}.log", gen.to_string()))
}

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

/// The `KvStore` stores string key/value pairs.
#[derive(Clone)]
pub struct KvStore<P: ThreadPool> {

    // current log file writer
    writer: Arc<Mutex<KvStoreWriter>>,
    // command store index
    index: Arc<SkipMap<String, CommandPos>>,
    thread_pool: P,
    reader_pool: Arc<ArrayQueue<KvStoreReader>>,
}

impl <P: ThreadPool> KvStore<P> {
    /// create KvStore through screen given path
    pub fn open(path: impl Into<PathBuf>, concurrency: u32) -> Result<Self> {
        
        let path:Arc<PathBuf> = Arc::new(path.into());
        // if path not exists
        if !path.exists() {
            fs::create_dir_all(&*path)?;
        }

        let mut readers = BTreeMap::new();
        let index = Arc::new(SkipMap::new());
        
        let mut uncompacted = 0;
        let gen_list = sorted_gen_list(&path)?;

        for &gen in &gen_list {
            let mut reader = BufReaderWithPos::new(File::open(log_path(&path, gen))?)?;
            uncompacted += load(gen, &mut reader, &*index)?;
            readers.insert(gen, reader);
        }
        let current_gen = gen_list.last().unwrap_or(&0) + 1;
        let writer = new_log_file(&path, current_gen)?;
        readers.insert(current_gen, BufReaderWithPos::new(File::open(log_path(&path, current_gen))?)?);

        let check_point = Arc::new(AtomicU64::new(0));

        let reader = KvStoreReader {
            path: Arc::clone(&path),
            check_point,
            readers: RefCell::new(readers),
        };

        let writer = KvStoreWriter {
            reader: reader.clone(),
            writer,
            current_gen,
            uncompacted,
            path: Arc::clone(&path),
            index: Arc::clone(&index),
        };

        let thread_pool = P::new(concurrency)?;
        let reader_pool = Arc::new(ArrayQueue::new(concurrency as usize));
        for _ in 1..concurrency {
            reader_pool.push(reader.clone()).map_err(|_| "push reader error").unwrap();
        }
         reader_pool.push(reader).map_err(|_| "push reader error").unwrap();

        Ok(KvStore {
            thread_pool,
            reader_pool,
            index,
            writer: Arc::new(Mutex::new(writer)),
        })
    }

}
impl <P: ThreadPool>KvsEngine for KvStore<P> {

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&self, key: String, value: String) -> Box<dyn Future<Output = Result<Result<()>>> + Send>{
        let writer = self.writer.clone();
        let (tx, rx) = oneshot::channel();
        self.thread_pool.spawn(move || {
            let res = writer.lock().unwrap().set(key, value);
            if tx.send(res).is_err() {
                error!("Receiving end is dropped");
            }
        });
        Box::new(
            rx.map_err(KvsError::from)
        )
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&self, key: String) -> Box<dyn Future<Output = Result<Result<Option<String>>>> + Send> {
        let reader_pool = self.reader_pool.clone();
        let index = self.index.clone();
        let (tx, rx) = oneshot::channel();
        self.thread_pool.spawn(move || {
            let res = (|| {
                if let Some(cmd_pos) = index.get(&key) {
                    let reader = reader_pool.pop().unwrap();
                    let res = if let Command::Set { value, .. } =
                        reader.read_command(*cmd_pos.value())?
                    {
                        Ok(Some(value))
                    } else {
                        Err(KvsError::UnexpectedCommandType)
                    };
                    reader_pool.push(reader).map_err(|_| "push reader error").unwrap();
                    res
                } else {
                    Ok(None)
                }
            })();
            if tx.send(res).is_err() {
                error!("Receiving end is dropped");
            }
        });
        Box::new(
            rx.map_err(KvsError::from)
        )
    }

    /// Remove a given key.
    fn remove(&self, key: String) -> Box<dyn Future<Output = Result<Result<()>>> + Send> {
        let writer = self.writer.clone();
        let (tx, rx) = oneshot::channel();
        self.thread_pool.spawn(move || {
            let res = writer.lock().unwrap().remove(key);
            if tx.send(res).is_err() {
                error!("Receiving end is dropped");
            }
        });
        Box::new(
            rx.map_err(KvsError::from)
        )
    }

}

/// A reader for per thread
struct KvStoreReader {
    path: Arc<PathBuf>,
    // generation of the latest compaction file
    check_point: Arc<AtomicU64>,
    readers: RefCell<BTreeMap<u64, BufReaderWithPos<File>>>,
}

impl KvStoreReader {
    /// Close file handles with generation number less than check_point.
    fn close_stale_handles(&self) {
        let mut readers = self.readers.borrow_mut();
        while !readers.is_empty() {
            let first_gen = *readers.keys().next().unwrap();
            if first_gen >= self.check_point.load(Ordering::SeqCst) {
                break;
            }
            readers.remove(&first_gen);
        }
    }

    /// Read the log file at the given `CommandPos`.
    fn read_and<F, R>(&self, cmd_pos: CommandPos, f: F) -> Result<R>
    where
        F: FnOnce(io::Take<&mut BufReaderWithPos<File>>) -> Result<R>,
    {
        self.close_stale_handles();

        let mut readers = self.readers.borrow_mut();
        // Open the file if we haven't opened it in this `KvStoreReader`.
        // We don't use entry API here because we want the errors to be propogated.
        if !readers.contains_key(&cmd_pos.gen) {
            let reader = BufReaderWithPos::new(File::open(log_path(&self.path, cmd_pos.gen))?)?;
            readers.insert(cmd_pos.gen, reader);
        }
        let reader = readers.get_mut(&cmd_pos.gen).unwrap();
        reader.seek(SeekFrom::Start(cmd_pos.pos))?;
        let cmd_reader = reader.take(cmd_pos.len);
        f(cmd_reader)
    }

    // Read the log file at the given `CommandPos` and deserialize it to `Command`.
    fn read_command(&self, cmd_pos: CommandPos) -> Result<Command> {
        self.read_and(cmd_pos, |cmd_reader| {
            Ok(serde_json::from_reader(cmd_reader)?)
        })
    }
}


impl Clone for KvStoreReader {
    fn clone(&self) -> KvStoreReader {
        KvStoreReader {
            path: Arc::clone(&self.path),
            check_point: Arc::clone(&self.check_point),
            // don't use other KvStoreReader's readers
            readers: RefCell::new(BTreeMap::new()),
        }
    }
}

/// single writer in whole program
struct KvStoreWriter {
    path: Arc<PathBuf>,
    reader: KvStoreReader,
    writer: BufWriterWithPos<File>,
    current_gen: u64,
    // the number of bytes representing "stale" commands that could be
    // deleted during a compaction
    uncompacted: u64,
    index: Arc<SkipMap<String, CommandPos>>,
}

impl KvStoreWriter {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        let pos = self.writer.cursor_pos;
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        if let Command::Set { key, .. } = cmd {
            if let Some(old_cmd) = self.index.get(&key) {
                self.uncompacted += old_cmd.value().len;
            }
            self.index
                .insert(key, (self.current_gen, pos..self.writer.cursor_pos).into());
        }

        if self.uncompacted > COMPACTION_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            let cmd = Command::remove(key);
            let pos = self.writer.cursor_pos;
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;
            if let Command::Remove { key } = cmd {
                let old_cmd = self.index.remove(&key).expect("key not found");
                self.uncompacted += old_cmd.value().len;
                // the "remove" command itself can be deleted in the next compaction
                // so we add its length to `uncompacted`
                self.uncompacted += self.writer.cursor_pos - pos;
            }

            if self.uncompacted > COMPACTION_THRESHOLD {
                self.compact()?;
            }
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

    /// Clears stale entries in the log.
    fn compact(&mut self) -> Result<()> {
        // increase current gen by 2. current_gen + 1 is for the compaction file
        let compaction_gen = self.current_gen + 1;
        self.current_gen += 2;
        
        self.writer = new_log_file(&self.path, self.current_gen)?;

        let mut compaction_writer = new_log_file(&self.path, compaction_gen)?;

        let mut new_pos = 0; // pos in the new log file
        for entry in self.index.iter() {
            let len = self.reader.read_and(*entry.value(), |mut entry_reader| {
                Ok(io::copy(&mut entry_reader, &mut compaction_writer)?)
            })?;
            self.index.insert(
                entry.key().clone(),
                (compaction_gen, new_pos..new_pos + len).into(),
            );
            new_pos += len;
        }
        compaction_writer.flush()?;

        self.reader
            .check_point
            .store(compaction_gen, Ordering::SeqCst);
        self.reader.close_stale_handles();

        // remove stale log files
        // Note that actually these files are not deleted immediately because `KvStoreReader`s
        // still keep open file handles. When `KvStoreReader` is used next time, it will clear
        // its stale file handles. On Unix, the files will be deleted after all the handles
        // are closed. On Windows, the deletions below will fail and stale files are expected
        // to be deleted in the next compaction.

        let stale_gens = sorted_gen_list(&self.path)?
            .into_iter()
            .filter(|&gen| gen < compaction_gen);
        for stale_gen in stale_gens {
            let file_path = log_path(&self.path, stale_gen);
            if let Err(e) = fs::remove_file(&file_path) {
                error!("{:?} cannot be deleted: {}", file_path, e);
            }
        }
        self.uncompacted = 0;

        Ok(())
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    cursor_pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let cursor_pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            cursor_pos,
        })
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.cursor_pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.cursor_pos = self.writer.seek(pos)?;
        Ok(self.cursor_pos)
    }
}

struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

/// create log file
fn new_log_file(
    path: &Path,
    gen: impl ToString,
) -> Result<BufWriterWithPos<File>> {
    let path = log_path(&path, gen);
    let writer = BufWriterWithPos::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    )?;
    Ok(writer)
}
/// Return log folder exist gen log file list
fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {
    let mut gen_list: Vec<u64> = fs::read_dir(&path)?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) } )
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| path.file_stem().map(OsStr::to_str).flatten().map(str::parse::<u64>))
        .flatten()
        .collect();
    gen_list.sort_unstable();
    Ok(gen_list)
}
/// load whole log file
fn load(
    gen: u64,
    reader: &mut BufReaderWithPos<File>,
    index: &SkipMap<String, CommandPos>,
) -> Result<u64> {
    // To make sure we read from the beginning of the file
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    let mut uncompacted = 0;
    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, .. } => {
                if let Some(old_cmd) = index.get(&key) {
                    uncompacted += old_cmd.value().len;
                }
                index.insert(key, (gen, pos..new_pos).into());
            }
            Command::Remove { key } => {
                if let Some(old_cmd) = index.remove(&key) {
                    uncompacted += old_cmd.value().len;
                }
                uncompacted += new_pos - pos;
            }
        }
        pos = new_pos;
    }
    Ok(uncompacted)
}