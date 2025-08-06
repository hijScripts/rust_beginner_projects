use std::io; // input/output library, comes from standard library
use rand::Rng; // Rng trait, comes from rand library

// open local doc with all info for dependencies used: cargo doc --open

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // creates number local to current thread of execution and seeded by the OS.
                                                               // then generates a range from 1-100 inclusive of the lower and upper.

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // vars are immutable by default, 'mut' allows us to mutate them.

    io::stdin()
        .read_line(&mut guess) // gets user input and binds to 'guess' var. The '&' indicates the argument is a reference, 
                               // which is also immutable by default. 
        .expect("Failed to read line"); // Result returns either 'Err' or 'Ok', this line crashes program if 'Err' is received.

    println!("You guessed: {guess}");
}