// Example of delcaring data type
// fn main() {

//     // Without the ': u32', compile-time error will generate
//     let guess: u32 = "42".parse().expect("Not a number!");

//     println!("{guess}")
// }

/* Scalar Data Types 
Integers
Floating-point numbers
Booleans
Characters
*/

/* Types of Integers (Signed: i (for negatives & positives), 
                      Unsigned: u (for only positives))
Length
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
architecture dependent	isize	usize

inclusive ranges, n = amt of bits
unsigned range: 0 to 2n − 1
signed range: −(2^(n − 1)) to 2^(n − 1) − 1
*/

/* Types of Floating Points
Length
32-bit  f32
64-bit  f64

f64 considered as fast as f32 nowadays with more precision.
*/
// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32

//     println!("{x}, {y}")
// }

/* Numeric Operations */
// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;
// }

/* Boolean Type (1 byte in size) */
// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }

/* Char Type */
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

/* Compound Types 
Tuples
Array
*/

/* Tuples */
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

// Getting individual value out of a tuple with a pattern
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

// Getting individual value out of a tuple with indexing
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

/* Array */