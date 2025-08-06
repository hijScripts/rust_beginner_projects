use std::io; // input/output library, comes from standard library

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // vars are immutable by default, 'mut' allows us to mutate them.

    io::stdin()
        .read_line(&mut guess) // gets user input and binds to 'guess' var. The '&' indicates the argument is a reference, 
                               // which is also immutable by default. 
        .expect("Failed to read line"); // Result returns either 'Err' or 'Ok', this line crashes program if 'Err' is received.

    println!("You guessed: {guess}");
}