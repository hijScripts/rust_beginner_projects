use std::io;

fn main() {
    let mut input: String = String::new(); 

    io::stdin()
        .read_line(&mut input)      
        .expect("Failed to read line"); 

    let input: &str = input.trim();

    let value: u32 = match input {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        _ => 0,
    };

    println!("Value of '{input}' = {value}");
}