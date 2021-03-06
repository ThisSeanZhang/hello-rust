use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);


    loop {
        let mut guess = String::new();
        println!("Please input your number");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("input number: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            // .expect("Please type a number!");

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("your Win");
                break;
            },
        }
    }
}
