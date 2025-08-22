fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // since we are passing a reference to s1, it does not move ownership.
                                     // s1 is still valid after this call.

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}   // Here, s goes out of scope. But because s does not have ownership of what
    // it refers to, the String is not dropped.

// ** mutable references ** //
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// this wont run as it violates the borrowing rules. Can only have one mutable 
// reference to a piece of data in a particular scope.
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
}