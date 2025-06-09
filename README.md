# Guessing Game

This is a simple command-line guessing game written in Rust. The program generates a random number, and you try to guess it. After each guess, you will be told if your guess is too high, too low, or correct.

## How to Run

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Open a terminal and navigate to this project folder.
3. Build and run the game with:

   ```sh
   cargo run --release
   ```

   This command builds the game in release mode, which is optimized for better performance.

   If you want to run the game in debug mode (useful for development and testing), use:

   ```sh
   cargo run
   ```

   Debug mode builds faster but is less optimized.

   **Difference between release and debug mode:**  
   Release mode enables optimizations, making the program run faster and more efficiently, but takes longer to compile. Debug mode compiles quickly and is useful during development, but the resulting program runs slower.

4. Follow the on-screen instructions to play the game.

## Tutorial

This project is based on the official Rust guessing game tutorial:  
https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

## What I Learned

- How to set up a Rust project using Cargo.
- How to read user input and handle input errors.
- How to generate random numbers in Rust.
- How to use control flow (loops, conditionals) in Rust.
- How to compare values and provide feedback to the user.
- The difference between debug and release builds in Rust.
