pub fn run() {
    loop {
        println!("Again and Again");
        break;
    }

    let mut launch_counts = 10;

    while launch_counts != 0 {
        println!("{}", launch_counts);
        launch_counts -= 1;
    }

    println!("Ignition!!!!1!!");

    let mut numbers: Vec<i32> = vec![10, 20, 40, 100, 30, 450];

    for elements in numbers.iter() {
        println!("The value is: {}", elements);
    }

    for add in numbers.iter_mut() {
        *add *= 2;
        println!("Add by 2: {}", add);
    }

    println!("All arrays: {:?}", numbers);

    for nums in 1..1000 {
        if nums % 2 == 0 {
            if nums == 450 {
                break;
            }
            println!("Nums: {}", nums);
        }
    }

    for new in (1..100).rev() {
        if new % 2 == 0 {
            println!("New: {}", new);
        }
    }
}
