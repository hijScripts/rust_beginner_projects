// fn main() {
//     println!("Hello, world!");

//     another_function(5, 'h');
// }

// parameters speical vars the are apart of functions signature. 
// Interchangable with arguments that are concrete values
fn another_function(x: i32, unit_label: char) {
    println!("Another function. {x}, {unit_label}");
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

//statement
// fn main() {
//     let x = 5;
// }

//expression. Expressions don't end with ; as it turns it into a statement
// prevents a value from being returned
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// implicit returning in functions.
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}