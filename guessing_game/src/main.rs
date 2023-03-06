// Bring in input output from the standard library
use std::io;
// We added rand in cargo.toml and can now use it here
use rand::Rng;
// Bring in the ordering type which is an enum with the
// variants Less, Greater and Equal
use std::cmp::Ordering;

// "cargo doc --open" generates a webpage which will open in
// the browser which shows the documentation of included crates

fn main() {
    println!("Guess the number!");

    
    // Using the rand create to generate a number from 1 to 100
    // thread_rng is local to the current thread of execution
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // Variables in rust are immutable by default (cant be changed)
    // So we have to state mut to make them mutable
    // String::new() returns a new instance of String which is growable UTF-8 encoded bit of text
    // ::new indicates that new is an associated function of String
    let mut guess = String::new();

    // stdin() is a function of io and .read_line is a method of that
    // & makes guess a reference, and we have to declare it mutable
    // .expect handles an Err return from read_line and crashes the program

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    // we parse guess to an int so we can compare it with the
    // secret_number. trim removes whitespace
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    // cmp takes a reference to whatever you want to compare to
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}