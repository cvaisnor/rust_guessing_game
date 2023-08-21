use std::io; // standard library (std) and input/output (io) module
use rand::Rng; // external dependency (rand) and trait (Rng)
use std::cmp::Ordering; // comparison (cmp) module and Ordering enum, contains variants [Less, Greater, Equal]

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng() // selects a random number generator local to the current thread of execution, seeded by the operating system
        .gen_range(1..=100); // generates a random number between 1 and 100 (inclusive on lower and upper bounds)

    // println!("The secret number is: {secret_number}");

    loop {
        println!();
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess) // making the reference mutable
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catchall value, and will run the loop again
        };

        println!("You guessed: {guess}");
        println!();
    
        // match decides what to do with the returned variant of the Ordering enum
        // match is an expression made of arms, each arm has a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm's pattern
        match guess.cmp(&secret_number) { // compares two values and can be called on anything that can be compared
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"), // the match expression gets this arm and checks it againsts the others
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}