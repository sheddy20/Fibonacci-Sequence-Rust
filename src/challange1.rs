pub fn run() {
    let fahreinheit = 32;
    let temperature = temperature_converter(32);
    println!(
        "Temperature is: {} degrees celsius",
        fahreinheit - temperature * 5 / 9
    );
}

fn temperature_converter(normal_temp: i32) -> i32 {
    normal_temp
}
