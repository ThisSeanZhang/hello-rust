use crate::protocol::{Request, Response};
use crate::{KvsEngine, Result};
use log::{debug, info};
use serde_json::Deserializer;
use std::net::{SocketAddr};
use tokio::net::{TcpListener, TcpStream};

/// The server of a key value store.
pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E) -> Self {
        KvsServer { engine }
    }

    /// Run the server listening on the given address
    pub async fn run(self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(&addr).await?;
        info!("Listening on {}", &addr);
        loop {
            let (tcp, _) = listener.accept().await?;
            let engine = self.engine.clone();
            serve(engine, tcp).await?;
        }
    }
}

async fn serve<E: KvsEngine>(engine: E, tcp: TcpStream) -> Result<()> {

    let peer_addr = tcp.peer_addr()?;
    info!("accept connect {}", &peer_addr);
    
    let mut data = vec![];
    loop {
        let mut buffer = [0; 1024];
        tcp.readable().await?;
        info!("can read data from: {}", &peer_addr);
        
        match tcp.try_read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                info!("data: {}", std::str::from_utf8(&buffer[0..n]).unwrap());
                data.extend_from_slice(&buffer[0..n]);
                if buffer[n-2] == 13 && buffer[n-1] == 10 {
                    break;
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            },
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    info!("{}", std::str::from_utf8(&data).unwrap());
    let req_reader = Deserializer::from_slice(&data).into_iter::<Request>();
    // let (_, writer) = tcp.split();
        macro_rules! send_resp {
        ($resp:expr) => {{
            let resp = $resp;
            tcp.try_write(&serde_json::to_vec(&resp)?)?;
            // serde_json::to_writer(&mut writer, &resp)?;
            debug!("Response sent to {}: {:?}", peer_addr, resp);
        }};
    }

    for req in req_reader {
        let req = req?;
        debug!("Receive request from {}: {:?}", peer_addr, req);
        let result = match req {
            Request::Get { key } => match engine.get(key).await {
                Ok(data) => Response::Get(data),
                Err(e) => Response::Err(format!("{}", e)),
            },
            Request::Set { key, value } => match engine.set(key, value).await {
                Ok(_) => Response::Set,
                Err(e) => Response::Err(format!("{}", e)),
            },
            Request::Remove { key } => match engine.remove(key).await {
                Ok(_) => Response::Remove,
                Err(e) => Response::Err(format!("{}", e)),
            },
        };
        
        tcp.writable().await?;
        send_resp!(result);
    }
    Ok(())
}
