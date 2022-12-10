use std::process::exit;
use std::env::current_dir;

use clap::Parser;
use kvs::{KvStore, KvsError, Result};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(subcommand_required = true)]
struct Args {

    #[command(subcommand)]
    action: Action,

}

#[derive(clap::Subcommand, Debug)]
enum Action {

    /// Set the value of a string key to a string
    Set {
        /// A string key
        #[arg(name="key", required=true)]
        key: String,
        /// The string value of the key
        #[arg(name="value", required=true)]
        value: String 
    },
    /// Get the string value of a given string key
    Get {
        /// A string key
        #[arg(name="key", required=true)]
        key: String,
    },
    /// Remove a given key
    Rm {
        /// A string key
        #[arg(name="key", required=true)]
        key: String,
    }
}
fn main() -> Result<()> {

    let args = Args::parse();

    match args.action {
    Action::Set { key, value } => {
        let mut store = KvStore::open(current_dir()?)?;
        store.set(key, value)?;
    },
    Action::Get { key } => {
        let mut store = KvStore::open(current_dir()?)?;
        if let Some(value) = store.get(key)? {
            println!("{}", value);
        } else {
            println!("Key not found");
        }
    },
    Action::Rm { key } => {
        let mut store = KvStore::open(current_dir()?)?;
        match store.remove(key) {
            Ok(()) => {}
            Err(KvsError::KeyNotFound) => {
                println!("Key not found");
                exit(1);
            }
            Err(e) => return Err(e),
        }
    },
    }

    Ok(())
}
