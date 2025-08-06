use std::cmp::Ordering; // Ordering is an enum, results in Less, Greater, or Equal.
use std::io; // input/output library, comes from standard library

use rand::Rng; // Rng trait, comes from rand library

// open local doc with all info for dependencies used: cargo doc --open

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // creates number local to current thread of execution and seeded by the OS.
                                                               // then generates a range from 1-100 inclusive of the lower and upper.
                                                               // defaults to an i32 type number.

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // vars are immutable by default, 'mut' allows us to mutate them.

        io::stdin()
            .read_line(&mut guess) // gets user input and binds to 'guess' var. The '&' indicates the argument is a reference, 
                                // which is also immutable by default. 
            .expect("Failed to read line"); // Result returns either 'Err' or 'Ok', this line crashes program if 'Err' is received.

        let guess: u32 = match guess.trim().parse() { // converts guess to a number as String can not be compared to i32.
                                                // parse() allows for a string to be converted to diff type.
            Ok(num) => num,
            Err(_) => continue, // `_` is a catch-all value
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // can be used on anything that is comparable, compares two references.
                                        // returns Ordering::[1 of three results]
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    }
}