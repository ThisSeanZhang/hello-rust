use std::{env, fs, process};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let args = [String::from("target\\debug\\minigrep.exe"), String::from("something"), String::from("poem.txt")];
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
    Ok(())
}


struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename
        })
    }
}