fn main() {

    // Without the ': u32', compile-time error will generate
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{guess}")
}
