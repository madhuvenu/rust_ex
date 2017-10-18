extern crate rand;

use rand::Rng;

fn simple_if() {
    let condition : bool = true;
    // Notice how there are no paranthesis between condition like python but 
    // there are braces between the if/else like in C++
    if condition {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
}

fn multiple_if() {
    const COMPARATOR : u32 = 100;
    let number : u32 = rand::thread_rng().gen_range(1, 100);
    // Multiple if / else statements
    if number < COMPARATOR {
        println!("Number {} too small", number);
    } else if number > COMPARATOR {
        println!("Number {} too big", number);
    } else {
        println!("Number {} just right", number);
    }
}


fn let_and_if() {
    const NUM_THRESHOLD : u32 = 50;
    // select a random number in the range from 1 to 100
    let random : u32 = rand::thread_rng().gen_range(1, 100);

    // Now assign a value based on the random number
    let number = if random < NUM_THRESHOLD {
        println!("Using the random number as is.");
        // Notice how there are no semicolons
        random
    } else {
        println!("Trimming the random number as its large");
        // The else block should return a value of the same type
        random % NUM_THRESHOLD
    };
    println!("Number assigned the value: {}.", number);
}

fn unconditional_looping() {
    let mut count = 0;
    loop {
        println!("What are we doing here.");
        count = count + 1;
        if count > 4 {
            break;
        }
    }
    println!("Count for loop is : {}", count);
}

fn conditional_while() {
    let mut count = 0;
    while count < 5 {
        println!("What are we doing here.");
        count = count + 1;
    }
    println!("Count for while loop is : {}", count);
}

fn conditional_for() {
    // Using a for loop with a range instead of a counting while loop
    for value in 0..5 {
        println!("What are we doing here. ({})", value);
    }
}

fn iterating_for() {
    let a = [10, 100, 200];
    // Loop over the elements of the array
    for item in a.iter() {
        println!("{} ", item);
    }
}

fn main() {
    simple_if();
    multiple_if();
    let_and_if();

    unconditional_looping();
    conditional_while();
    conditional_for();
    iterating_for();    
}
