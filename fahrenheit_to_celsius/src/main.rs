use std::io;

fn main() {
    let fahrenheit_temp: f64 = get_fahrenheit_temp();
    println!("You entered {fahrenheit_temp}F");

    let celsius_temp: f64 = convert_to_celsius(fahrenheit_temp);
    println!("{fahrenheit_temp}F is equal to {celsius_temp:.2}C");
}

fn get_fahrenheit_temp() -> f64 {

    println!("Please input the temperature in Fahrenheit.");
    
    let mut temp: String = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");
    
    let temp: f64 = match temp.trim().parse() {
        Ok(temp) => temp,
        Err(_) => todo!(),
    };

    temp
}

fn convert_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0/9.0)
}