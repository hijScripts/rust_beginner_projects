// getting first word of a string without slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// problem with above
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
}

// slices
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // these two statements are the same
    let slice = &s[0..2];
    let slice = &s[..2];

    // these two are also the same
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    // as are these
    let slice = &s[0..len];
    let slice = &s[..];
}

// rewriting first function to use slices
//fn first_word(s: &str) -> &str { better way to write it as it allows for operations on
// any string slice, not just String
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// now shows error
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {word}");
}
