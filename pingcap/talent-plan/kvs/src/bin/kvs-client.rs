use clap::Parser;
use kvs::{KvsClient, Result};
use std::net::SocketAddr;
use std::process::exit;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(subcommand_required = true)]
#[command(name = "kvs-client")]
struct Args {

    #[command(subcommand)]
    action: Action,

}

#[derive(clap::Subcommand, Debug)]
enum Action {

    /// Set the value of a string key to a string
    Set {
        /// Sets the server address
        #[arg(
          long,
          value_name = ADDRESS_FORMAT,
          default_value = DEFAULT_LISTENING_ADDRESS,
        )]
        addr: SocketAddr,
        /// A string key
        #[arg(name = "key", required=true)]
        key: String,
        /// The string value of the key
        #[arg(name = "value", required=true)]
        value: String
    },
    /// Get the string value of a given string key
    Get {
        /// Sets the server address
        #[arg(
          long,
          value_name = ADDRESS_FORMAT,
          default_value = DEFAULT_LISTENING_ADDRESS,
        )]
        addr: SocketAddr,
        /// A string key
        #[arg(name = "key", required=true)]
        key: String,
    },
    /// Remove a given key
    Rm {
      /// Sets the server address
      #[arg(
        long,
        value_name = ADDRESS_FORMAT,
        default_value = DEFAULT_LISTENING_ADDRESS,
      )]
      addr: SocketAddr,
      /// A string key
      #[arg(name = "key", required=true)]
      key: String,
    }
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    match args.action {
        Action::Get { key, addr } => {
            let mut client = KvsClient::connect(addr)?;
            if let Some(value) = client.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Action::Set { key, value, addr } => {
            let mut client = KvsClient::connect(addr)?;
            client.set(key, value)?;
        }
        Action::Rm { key, addr } => {
            let mut client = KvsClient::connect(addr)?;
            client.remove(key)?;
        }
    }
    Ok(())
}