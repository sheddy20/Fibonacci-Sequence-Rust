pub fn run() {
    let mut x = 5;
    println!("the value of X is: {}", x);
    x = 10;
    println!("the value of X is: {}", x);

    const ID: i32 = 400;
    println!("ID: {}", ID);

    const MAX_POINTS: u32 = 1000_000;
    println!("Max Points: {}", MAX_POINTS);

    //Shadowing
    let big_number = 100;
    let big_number = big_number + 200;
    let big_number = big_number * 450;
    println!("Big number: {}", big_number);

    let spaces = "       ";
    let spaces = spaces.len();
    println!("The Space is: {}", spaces);
}
