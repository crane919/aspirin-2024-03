//! A simple number guessing game

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Get input from user from standard in
fn get_input() -> i32 {
    println!("Please input your guess");

    let mut input = String::new();
    // asign the value of std to input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // parse the input and check that it is valid
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid entry."),
    }
}

/// Handles main game logic, and tells user if their guess is correct or not
fn main() {
    println!("Guess the number!");

    // Generate target number randomly
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop the guessing until user guesses correctly
    loop {
        let guess = get_input(); // call the input function
        print!("You guessed: {}. ", guess);

        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("That is correct!");
                break;
            }
            Ordering::Greater => println!("You're guess is too low."),
            Ordering::Less => println!("You're guess is too high."),
        }
    }
}
