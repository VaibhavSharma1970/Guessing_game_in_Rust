use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guess the Number game!");
    
    // Generate a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);

    loop {
        println!("Please enter your guess:");

        // Create a mutable variable to store the player's guess.
        let mut guess = String::new();

        // Read the player's input.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input to a number (integer).
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Compare the guess to the secret number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the correct number: {}", secret_number);
                break;
            }
        }
    }
}
