use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &String::from("something");
    let filename = &String::from("poem.txt");

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

}
