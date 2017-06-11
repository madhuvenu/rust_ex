// Import the rand crate (specified in Cargo.toml)
extern crate rand;

// Import things from other namespaces
use std::io;

fn add(x:i32, y:i32) -> i32 {
    return x + y;
}

fn read_num() -> i32 {
    let mut num = String::new();
    // Accept the number string from the command line
    io::stdin().read_line(&mut num)
        .expect("Failed to read a number!");

    // Parse the string and convert it into a valid number
    let num : i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            panic!("You did not enter a valid integer ({}).", err);
        }
    };
    // Return the integer that was entered if everything went well
    return num;
}

fn main() {
    println!("\nPlease specify the two numbers that need to be added.");
    println!("First Number: ");
    let num1 = read_num();
    
    println!("Second Number: ");
    let num2 = read_num();

    let sum = add(num1, num2);
    println!("Sum : {} + {} = {}", num1, num2, sum);
}
