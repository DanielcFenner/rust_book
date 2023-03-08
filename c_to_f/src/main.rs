use std::io;

fn main() {
    println!("Press F to input Fahrenheit, C for Celcius");

    // read and trim input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("you messed up");
    let input: &str = input.trim();

    println!("Input temperature to convert");

    // read, trim and convert input to f64
    let mut temperature_input: String = String::new();
    io::stdin().read_line(&mut temperature_input).expect("whoops");
    let temperature_input: f64 = temperature_input.trim().parse().unwrap();

    // convert to F or C depending on previous input
    if input == "f" || input == "F" {
        let result: f64 = (temperature_input - 32.0) * 0.5556;
        println!("{temperature_input} Fahrenheit is {result} Celcius");
    } else {
        let result: f64 = temperature_input * 1.8 + 32.0;
        println!("{temperature_input} Celcius is {result} Fahrenheit")c;
    }
}
