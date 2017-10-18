use std::io;

fn main() {
    println!("Input a string:");
    
    let mut str = String::new();
    // Accept a string from the command line
    io::stdin().read_line(&mut str)
        .expect("Failed to read a string!");
    // Trim the new line at the end 
    let str = str.trim();
    
    // Output the string
    println!("You entered {}!", str);
}
