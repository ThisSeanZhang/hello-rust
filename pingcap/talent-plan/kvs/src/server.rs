use crate::protocol::{Request, Response};
use crate::thread_pool::ThreadPool;
use crate::{KvsEngine, Result, KvsError};
use futures::{Future, FutureExt, TryFutureExt};
use log::{debug, error};
use serde_json::Deserializer;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};
use std::io::{BufReader, BufWriter, Write};
use std::net::{SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use std::process::Output;

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
    pub fn run(self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(&addr);
        let server = listener.map_ok(
            |listen| async {
                loop {
                    let (tcp, _) = listen.accept().await.unwrap();
                    let engine = self.engine.clone();
                    serve(engine, tcp).map_err(|e| error!("Error on serving client: {}", e));
                }
            }
        );
            
        tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(server);
        Ok(())
    }
}

// fn serve<E: KvsEngine>(engine: E, tcp: TcpStream) -> Result<()> {
//     let peer_addr = tcp.peer_addr()?;
//     let reader = BufReader::new(&tcp);
//     let mut writer = BufWriter::new(&tcp);
//     let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

//     macro_rules! send_resp {
//         ($resp:expr) => {{
//             let resp = $resp;
//             serde_json::to_writer(&mut writer, &resp)?;
//             writer.flush()?;
//             debug!("Response sent to {}: {:?}", peer_addr, resp);
//         }};
//     }

//     for req in req_reader {
//         let req = req?;
//         debug!("Receive request from {}: {:?}", peer_addr, req);
//         let result = match req {
//             Request::Get { key } => match engine.get(key) {
//                 Ok(data) => Response::Get(data),
//                 Err(e) => Response::Err(format!("{}", e)),
//             },
//             Request::Set { key, value } => match engine.set(key, value) {
//                 Ok(_) => Response::Set,
//                 Err(e) => Response::Err(format!("{}", e)),
//             },
//             Request::Remove { key } => match engine.remove(key) {
//                 Ok(_) => Response::Remove,
//                 Err(e) => Response::Err(format!("{}", e)),
//             },
//         };
//         send_resp!(result);
//     }
//     Ok(())
// }

fn serve<E: KvsEngine>(engine: E, tcp: TcpStream) -> Result<()> {

    let mut buffer = [0; 1024];
    tcp.readable(&mut buffer);
    // let (read_half, mut write_half) = tcp.split();
    
    // let length_delimited = FramedRead::new(read_half, LengthDelimitedCodec::new());
    // // let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();
    // length_delimited.read_buffer()
    // // Deserialize frames
    // let mut deserialized = tokio_serde::SymmetricallyFramed::new(
    //     length_delimited,
    //     SymmetricalJson::<Value>::default(),
    // );

    // let resp_stream = read_json
    //     .map_err(KvsError::from)
    //     .and_then(
    //         move |req| -> Box<dyn Future<Output = Response> + Send> {
    //             match req {
    //                 Request::Get { key } => Box::new(engine.get(key).map(Response::Get)),
    //                 Request::Set { key, value } => {
    //                     Box::new(engine.set(key, value).map(|_| Response::Set))
    //                 }
    //                 Request::Remove { key } => {
    //                     Box::new(engine.remove(key).map(|_| Response::Remove))
    //                 }
    //             }
    //         },
    //     )
    //     .then(|resp| -> Result<Response> {
    //         match resp {
    //             Ok(resp) => Ok(resp),
    //             Err(e) => Ok(Response::Err(format!("{}", e))),
    //         }
    //     });
    // let write_json = WriteJson::new(FramedWrite::new(write_half, LengthDelimitedCodec::new()));
    // write_json
    //     .sink_map_err(KvsError::from)
    //     .send_all(resp_stream)
    //     .map(|_| ())
}
