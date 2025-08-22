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

// does not run as it violates the borrowing rules. Cannot have a mutable reference
// while immutable references are active.
fn main() {
        let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
}

// this runs as it does not violate the borrowing rules. r1 and r2 are not 
// used after this point.
fn main() {
        let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}