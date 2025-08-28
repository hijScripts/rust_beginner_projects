use std::io;

fn main() {
    let mut input: String = String::new(); // vars are immutable by default, 'mut' allows us to mutate them.

    io::stdin()
        .read_line(&mut input)      
        .expect("Failed to read line"); 

    let mut input: &str = input.trim();

    let value: u32 = match input {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        _ => 0,
    };

    println!("Value of '{input}' = {value}");
}