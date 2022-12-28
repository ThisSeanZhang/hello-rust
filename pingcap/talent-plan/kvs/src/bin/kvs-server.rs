use clap::{Parser, ValueEnum};
use kvs::*;
use kvs::thread_pool::{ThreadPool, RayonThreadPool};
use log::LevelFilter;
use log::{error, info, warn};
use std::env::current_dir;
use std::fs;
use std::net::SocketAddr;
use std::process::exit;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "kvs-server")]
struct Args {

  /// Sets the listening address
  #[arg(value_name = "IP:PORT",
    long,
    default_value = "127.0.0.1:4000",
  )]
  addr: SocketAddr,

  /// Sets the storage engine
  #[arg(
    value_enum,
    long,
    default_value_t = Engine::Kvs,
  )]
  engine: Engine,

}

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Engine {
  /// Key/Value Store
  Kvs,
  /// Sled Store
  /// 
  /// [sled](https://github.com/spacejam/sled)
  /// the champagne of beta embedded databases
  Sled,
}
impl FromStr for Engine {
  type Err = String;
  fn from_str(input: &str) -> std::result::Result<Engine, Self::Err> {
      match input {
          "Kvs"  => Ok(Engine::Kvs),
          "Sled" => Ok(Engine::Sled),
          _=> Err(input.into()),
      }
  }
}

fn main() {

    env_logger::builder().filter_level(LevelFilter::Info).init();
    let args = Args::parse();
    
    let start_step = || -> Result<()> {
      check_and_cache_engine(args.engine)?;
      run(args)?;
      Ok(())
    };

    if let Err(e) = start_step() {
        error!("{}", e);
        exit(1);
    }
}

fn run(opt: Args) -> Result<()> {
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    info!("Storage engine: {:?}", opt.engine);
    info!("Listening on {}", opt.addr);

    let concurrency = num_cpus::get() as u32;
    let pool = RayonThreadPool::new(concurrency)?;

    match opt.engine {
        Engine::Kvs => run_with_engine(KvStore::open(current_dir()?, concurrency)?, pool, opt.addr),
        Engine::Sled => run_with_engine(SledKvsEngine::new(sled::open(current_dir()?)?, concurrency), pool, opt.addr),
    }
}

fn run_with_engine<E: KvsEngine, P: ThreadPool>(engine: E, pool: P, addr: SocketAddr) -> Result<()> {
    let server = KvsServer::new(engine, pool);
    server.run(addr)
}

fn check_and_cache_engine(engine: Engine) -> Result<()> {
  let previous_engine_path = current_dir()?.join("engine");
  let mut previous_engine: Option<Engine> = None;
  if previous_engine_path.exists() {
    previous_engine = match fs::read_to_string(&previous_engine_path)?.parse() {
      Ok(engine) => Some(engine),
      Err(e) => {
          warn!("The content of engine file is invalid: {}", e);
          None
      }
    }
  }
  match previous_engine {
    None => {
      // write engine to engine file
      fs::write(previous_engine_path, format!("{:?}", engine))?;
      Ok(())
    },
    Some(e) if e == engine => {
      Ok(())
    },
    Some(_) => {
      Err(KvsError::WrongEngine)
    }
  }
}