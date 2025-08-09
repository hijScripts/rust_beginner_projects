// immutable var x
// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// mutable var x
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// const
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// Shadowing
fn main() {

    // Starts with value of 5
    let x = 5;

    // Then becomes 6
    let x = x + 1;

    {
        // Then becomes 12 only within local scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // Reverts back to 6, the previous value set
    println!("The value of x is: {x}");

    // this works and allows to reuse the same name and 
    // change immutable variable's value & data type
    // let spaces = "   ";
    // let spaces = spaces.len();

    // This does not work and generates compile time error
    // due to changing mutable variables data type
    // let mut spaces = "   ";
    // spaces = spaces.len();
}