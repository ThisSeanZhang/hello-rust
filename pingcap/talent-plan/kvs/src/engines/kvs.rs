use std::{
    collections::{HashMap, BTreeMap},
    path::{PathBuf, Path},
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write, Seek, SeekFrom, self, Read, BufReader}, ffi::OsStr
};

use serde_json::Deserializer;

use crate::{error::Result, KvsError, KvsEngine};
use crate::command::{Command, CommandPos};

fn log_path(path: &Path, gen: impl ToString) -> PathBuf {
    path.join(format!("{}.log", gen.to_string()))
}

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

/// The `KvStore` stores string key/value pairs.
pub struct KvStore {
    // directory for the log and other data.
    path: PathBuf,

    // current log file writer
    writer: BufWriterWithPos<File>,
    // gen reader dict
    readers: HashMap<u64, BufReaderWithPos<File>>,
    // command store index
    index: BTreeMap<String, CommandPos>,
    current_gen: u64,
    uncompacted: u64,
}

impl KvStore {
    /// create KvStore through screen given path
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        
        let path:PathBuf = path.into();
        // if path not exists
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        let mut uncompacted = 0;
        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();
        let gen_list = sorted_gen_list(&path)?;

        for &gen in &gen_list {
            let mut reader = BufReaderWithPos::new(File::open(log_path(&path, gen))?)?;
            uncompacted += load(gen, &mut reader, &mut index)?;
            readers.insert(gen, reader);
        }
        let current_gen = gen_list.last().unwrap_or(&0) + 1;
        let writer = new_log_file(&path, current_gen)?;
        readers.insert(current_gen, BufReaderWithPos::new(File::open(log_path(&path, current_gen))?)?);

        Ok(KvStore {
            path,
            writer,
            readers,
            index,
            current_gen,
            uncompacted,
        })
    }

    fn compact(&mut self) -> Result<()> {
        let compaction_gen = self.current_gen + 1;
        self.current_gen += 2;
        // Write new data to a new file
        self.writer = new_log_file(&self.path, self.current_gen)?;
        self.readers.insert(self.current_gen, BufReaderWithPos::new(File::open(log_path(&self.path, self.current_gen))?)?);
        
        // compaction file
        let mut compaction_writer = new_log_file(&self.path, &compaction_gen)?;
        self.readers.insert(compaction_gen, BufReaderWithPos::new(File::open(log_path(&self.path, &compaction_gen))?)?);
        
        let mut new_pos = 0; // pos in the new log file.
        for cmd_pos in &mut self.index.values_mut() {
            let reader = self
                .readers
                .get_mut(&cmd_pos.gen)
                .expect("Cannot find log reader");
            if reader.pos != cmd_pos.pos {
                reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            }

            let mut entry_reader = reader.take(cmd_pos.len);
            let len = io::copy(&mut entry_reader, &mut compaction_writer)?;
            *cmd_pos = (compaction_gen, new_pos..new_pos + len).into();
            new_pos += len;
        }
        compaction_writer.flush()?;

        // remove stale log files.
        let stale_gens: Vec<_> = self
            .readers
            .keys()
            .filter(|&&gen| gen < compaction_gen)
            .cloned()
            .collect();
        
        for stale_gen in stale_gens {
            self.readers.remove(&stale_gen);
            fs::remove_file(log_path(&self.path, stale_gen))?;
        }
        self.uncompacted = 0;
        Ok(())
    }
}
impl KvsEngine for KvStore {



    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        let start_pos = self.writer.cursor_pos;
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        let end_pos = self.writer.cursor_pos;
        if let Command::Set { key, .. } = cmd {
            if let Some(old_cmd) = self.index.insert(key, (self.current_gen, start_pos..end_pos).into()) {
                self.uncompacted += old_cmd.len;
            }
        }

        if self.uncompacted > COMPACTION_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(cmd_pos) = self.index.get(&key) {
            let reader = self
                .readers
                .get_mut(&cmd_pos.gen)
                .expect("Cannot find log reader");
            reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            let cmd_reader = reader.take(cmd_pos.len);
            if let Command::Set { value, .. } = serde_json::from_reader(cmd_reader)? {
                Ok(Some(value))
            } else {
                Err(KvsError::UnexpectedCommandType)
            }
        } else {
            Ok(None)
        }
    }

    /// Remove a given key.
    fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            let cmd = Command::remove(key);
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;
            if let Command::Remove { key } = cmd {
                let delete_pos = self.index.remove(&key).expect("Key not found");
                self.uncompacted += delete_pos.len;
            }
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

}

// struct CommandLog {
//     file_path: Path,
//     seq: u64,

// }

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
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<u64> {
    // To make sure we read from the beginning of the file
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    let mut uncompacted = 0;
    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, .. } => {
                if let Some(old_cmd) = index.insert(key, (gen, pos..new_pos).into()) {
                    uncompacted += old_cmd.len;
                }
            }
            Command::Remove { key } => {
                if let Some(old_cmd) = index.remove(&key) {
                    uncompacted += old_cmd.len;
                }
                uncompacted += new_pos - pos;
            }
        }
        pos = new_pos;
    }
    Ok(uncompacted)
}