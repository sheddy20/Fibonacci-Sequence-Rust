extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input the number? ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Oops Error");

        let guess: u32 = match guess.trim().parse() {
            Ok(numbers) => numbers,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
