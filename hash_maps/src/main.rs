use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");

    // // .copied() returns Option<i32> instead of Option<&i32>.
    // // unwrap_or(0) returns the value inside the Option or 0 if it's None.
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    // // iterating
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{field_name}") // error: borrow of moved value: `field_name`
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // overwriting
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // only changing value if key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // only insert if the key has no value, in this case it doesn't
    // or_insert returns a mutable reference to the value for the key
    scores.entry(String::from("Yellow")).or_insert(50);

    // only insert if the key has no value, in this case it does
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    // updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}