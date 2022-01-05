use std::io;

pub fn run() {
    println!("Enter any number divisible by 4, 3, or 2....");
    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("Not a numbers");

    // if number < 5 {
    //     println!("Condition was true");
    // } else {
    //     println!("Condition was false");
    // }

    // if number != 0 {
    //     println!("The number was something other than zero");
    // }
    get_odds();

    let number: i32 = number.trim().parse().unwrap();

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let new_number = if condition { 10 } else { 12 };

    println!("The value of New Number is: {}", new_number);
}

fn get_odds() {
    let mut numbers = 0;

    while numbers <= 10 {
        if numbers % 2 == 0 {
            println!("Even Numbers: {}", numbers);
        }
        numbers += 1;
    }
}
