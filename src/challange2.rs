use std::io;

pub fn run() {
    println!("Nth terms of fib...");

    let mut terms = String::new();
    io::stdin()
        .read_line(&mut terms)
        .expect("Failed to read number");

    let terms: i32 = terms.trim().parse().unwrap();

    let mut num1 = 0;
    let mut num2 = 1;
    let mut counters = 0;

    if terms <= 0 {
        println!("Enter a positive number");
    } else if terms == 1 {
        println!("Fibonacci sequence up to: {}", terms);
        println!("{}", num1);
    } else {
        println!("Fibonacci Sequence:");
        while counters < terms {
            println!("{}", num1);
            let nth = num1 + num2;
            num1 = num2;
            num2 = nth;
            counters += 1;
        }
    }
}
