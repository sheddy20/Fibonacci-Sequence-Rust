extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guess the number.");

    let unknown_number = rand::thread_rng().gen_range(1..101);

    println!("The Unknown number is: {}", unknown_number);

    loop {
        println!("Input your number....");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to run");

        let number: u32 = match number.trim().parse() {
            Ok(new_number) => new_number,
            Err(_) => continue,
        };

        println!("You guessed: {}", number);

        match number.cmp(&unknown_number) {
            Ordering::Less => println!("Number too small"),
            Ordering::Greater => println!("Number too big"),
            Ordering::Equal => {
                println!("Hey you win");
                break;
            }
        }
    }
}
