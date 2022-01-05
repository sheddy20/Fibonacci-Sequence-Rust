pub fn run() {
    let final_gpa = calculate_tma(30, 40, 100);
    println!("GPA: {}", final_gpa);
}

fn calculate_tma(tma_1: i32, tma_2: i32, tma_3: i32) -> i32 {
    tma_1 * tma_2 + tma_3
}
