// Like import in python
extern crate rand;

// Like the using command in C++ for importing things from other namespaces.
use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("\nGuess the number please.");
    println!("Your Guess: ");

    // Generate the secret number that should match the guess
    let secret = rand::thread_rng().gen_range(1, 100);

    loop { 
        let mut guess = String::new();    
        // Accept the string from the user
        io::stdin().read_line(&mut guess)
            .expect("Failed to read a number!");

        // Parse it and convert it to a valid positive integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("You did not enter a positive integer ({}).", err);
                break;
            }
        };
        println!("You guessed: {}", guess);

        // Match it to the secret random number previously generated
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        }
    }
    println!("Secret number: {}", secret);
}
