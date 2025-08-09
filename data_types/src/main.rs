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