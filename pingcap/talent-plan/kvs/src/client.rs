use crate::protocol::{Request, Response};
use crate::{KvsError, Result};
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// Key value store client
pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?;
        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(tcp_reader)),
            writer: BufWriter::new(tcp_writer),
        })
    }

    /// Get the value of a given key from the server.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;
        let resp = Response::deserialize(&mut self.reader)?;
        match resp {
            Response::Get(data) => Ok(data),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
            _ => Err(KvsError::StringError("Invalid response".to_owned())),
        }
    }

    /// Set the value of a string key in the server.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;
        let resp:Response = Response::deserialize(&mut self.reader)?;
        match resp {
            Response::Set => Ok(()),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
            _ => Err(KvsError::StringError("Invalid response".to_owned())),
        }
    }

    /// Remove a string key in the server.
    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;
        let resp:Response = Response::deserialize(&mut self.reader)?;
        match resp {
            Response::Remove => Ok(()),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
            _ => Err(KvsError::StringError("Invalid response".to_owned())),
        }
    }
}