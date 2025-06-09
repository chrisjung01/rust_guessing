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
    loop {
        println!("Please enter your guess:");

        let mut guess: String = String::new();

        // Read user input from stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // In debug mode, print the guess
        if cfg!(debug_assertions) {
            println!("Your guess was: {}", guess.trim());
        }

        // Check if the input was exit and break the loop
        if guess.trim().eq_ignore_ascii_case("exit") {
            println!("Exiting the game.");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue; // Skip to the next iteration of the loop
            }
        };

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                println!("Too small!");
            }
            std::cmp::Ordering::Greater => {
                println!("Too big!");
            }
            std::cmp::Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }
}
