fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // since we are passing a reference to s1, it does not move ownership.
                                     // s1 is still valid after this call.

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}