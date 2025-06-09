use std::io;

use rand::Rng;

fn main() {
    let secret_number = rand::rng().random_range(1..100);

    // The secret number is only printed in debug mode
    if cfg!(debug_assertions) {
        println!("Debug mode is enabled");
        println!("The secret number is: {}", secret_number);
    }

    println!("Guess the number game!");

    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess was: {}", guess.trim());
}
