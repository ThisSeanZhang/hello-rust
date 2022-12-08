use std::process::exit;

use clap::Parser;

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
fn main() {

    let args = Args::parse();

   match args.action {
    Action::Set { key, value } => {
        eprintln!("unimplemented");
        exit(1);
    },
    Action::Get { key } => {
        eprintln!("unimplemented");
        exit(1);
    },
    Action::Rm { key } => {
        eprintln!("unimplemented");
        exit(1);
    },
   }


}
