// Given a list of integers, use a vector and return the median 
// (when sorted, the value in the middle position) and mode 
// (the value that occurs most often; a hash map will be helpful 
// here) of the list.

use std::collections::HashMap;

fn main() {
    let mut vec = vec![1, 4, 3, 4, 6, 5, 6, 7, 1, 9];

    vec.sort();

    let median = vec[vec.len() / 2];

    println!("{median}");

    let mut occurrences = HashMap::new();

    for &value in &vec {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    let (key_with_most_occurences, _) = occurrences
        .iter()
        .max_by_key(|&(_, value)| value) // Compares based on the value
        .unwrap(); // Assumes the HashMap is not empty

    println!("{key_with_most_occurences}");
}
