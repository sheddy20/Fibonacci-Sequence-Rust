pub fn run() {
    another_func(20, 30);

    let x = 10;

    let y = {
        let x = 20;
        x + 10
    };

    println!("The value of Y and X is: {} {}", y, x);

    let answer = five();
    println!("Five: {}", answer);

    let x = plus_one(5);
    println!("The value of X is: {}", x);
}

fn another_func(x: i32, y: i32) {
    println!("The value of X is: {}", x);
    println!("The value of Y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
