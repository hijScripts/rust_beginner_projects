// // // Ownership Rules
// // // First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

// // // Each value in Rust has an owner.
// // // There can only be one owner at a time.
// // // When the owner goes out of scope, the value will be dropped.

// // fn main() {
// //     {                      // s is not valid here, since it's not yet declared
// //         let s = "hello";   // s is valid from this point forward

// //         // do stuff with s
// //     }   

// //     let s1 = String::from("hello");
// //     let s2 = s1; // s1 is no longer valid here, since ownership has been moved to s2

// //     println!("{s1}, world!");

// //     // cloning allows this to function without moving ownership
// //     let s1 = String::from("hello");
// //     let s2 = s1.clone();

// //     println!("s1 = {s1}, s2 = {s2}"); 

// //     let x = 5;
// //     let y = x; // x is still valid here, since integers are stored on the stack, not heap.
// //                // Ints posses the Copy trait, which allows them to be copied rather than moved.   

// //     println!("x = {x}, y = {y}");

// //     // Types with the copy trait
// //     // All the integer types, such as u32.
// //     // The Boolean type, bool, with values true and false.
// //     // All the floating-point types, such as f64.
// //     // The character type, char.
// //     // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
// // }

// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                         // ... and so is no longer valid here
//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // Because i32 implements the Copy trait,
//                                     // x does NOT move into the function,
//                                     // so it's okay to use x afterward.

// } // Here, x goes out of scope, then s. However, because s's value was moved,
//   // nothing special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

fn main() {
    let s1 = String::from("hello");

    let s2 = s1;

    println!("{s2}");
    println!("{s1}"); // this will error as s1 is no longer valid
}