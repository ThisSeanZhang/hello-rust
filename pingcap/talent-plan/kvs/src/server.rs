use crate::protocol::{Request, Response};
use crate::thread_pool::ThreadPool;
use crate::{KvsEngine, Result};
use log::{debug, error};
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

/// The server of a key value store.
pub struct KvsServer<E: KvsEngine, P: ThreadPool> {
    engine: E,
    pool: P,
}

impl<E: KvsEngine, P: ThreadPool> KvsServer<E, P> {
    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E, pool: P) -> Self {
        KvsServer { engine, pool }
    }

    /// Run the server listening on the given address
    pub fn run<A: ToSocketAddrs>(self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            let engine = self.engine.clone();
            self.pool.spawn(move || match stream {
                Ok(stream) => {
                    if let Err(e) = serve(engine, stream) {
                        error!("Error on serving client: {}", e);
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            })
        }
        Ok(())
    }
}

fn serve<E: KvsEngine>(engine: E, tcp: TcpStream) -> Result<()> {
    let peer_addr = tcp.peer_addr()?;
    let reader = BufReader::new(&tcp);
    let mut writer = BufWriter::new(&tcp);
    let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

    macro_rules! send_resp {
        ($resp:expr) => {{
            let resp = $resp;
            serde_json::to_writer(&mut writer, &resp)?;
            writer.flush()?;
            debug!("Response sent to {}: {:?}", peer_addr, resp);
        }};
    }

    for req in req_reader {
        let req = req?;
        debug!("Receive request from {}: {:?}", peer_addr, req);
        let result = match req {
            Request::Get { key } => match engine.get(key) {
                Ok(data) => Response::Get(data),
                Err(e) => Response::Err(format!("{}", e)),
            },
            Request::Set { key, value } => match engine.set(key, value) {
                Ok(_) => Response::Set,
                Err(e) => Response::Err(format!("{}", e)),
            },
            Request::Remove { key } => match engine.remove(key) {
                Ok(_) => Response::Remove,
                Err(e) => Response::Err(format!("{}", e)),
            },
        };
        send_resp!(result);
    }
    Ok(())
}
