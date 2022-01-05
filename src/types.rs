pub fn run() {
    //Integer
    let age = 20;
    println!("{}", age);
    let new_age: i32 = 23;
    println!("{}", new_age);

    //Floating Point
    let x = 64.0;
    println!("{}", x);
    let y: f64 = 100.5;
    println!("{}", y);

    //Numerical Operations
    let sum = 5 + 10;
    println!("Sum: {}", sum);

    let difference: f64 = 9.4 - 4.3;
    println!("Difference: {}", difference);

    let product = 4 * 10;
    println!("Product: {}", product);

    const QUOTIENTS: f32 = 56.7 / 32.2;
    println!("Quotients: {}", QUOTIENTS);

    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);

    let is_married = true;
    println!("{}", is_married);
    //Boolean Types
    let is_online: bool = false;
    println!("{}", is_online);

    //Character Types
    let c = 'Z';
    let z = ' ';
    let cat_eye = ' ';
    println!("{:?}", (c, z, cat_eye));

    //Compound Types

    //Tuple
    let tup: (i32, f64, u8) = (500, 45.0, 8);
    println!("Tuples: {:?}", tup);

    //Destructuring tuple
    let (first, second, third) = tup;
    //First
    println!("The value of first is: {}", first);
    //Second
    println!("The value of second is: {}", second);
    //Third
    println!("The value of third is: {}", third);

    //Dot Notation tuple
    let first_tup = tup.0;
    println!("{}", first_tup);
    let second_tup = tup.1;
    println!("{}", second_tup);
    let third_tup = tup.2;
    println!("{}", third_tup);

    //Arrays
    let arrays = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", arrays);

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for month in months {
        println!("Months: {}", month);
    }

    let first_months = months[0];
    println!("First Months: {}", first_months);

    //Functions
}
