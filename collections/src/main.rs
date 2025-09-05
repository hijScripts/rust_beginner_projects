// Creating a new vector    
let v: Vec<i32> = Vec::new();

// Vector macro
let v = vec![1, 2, 3];

// Updating a vector
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);

// Referencing elements
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}

// for loop
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}

// for loop to change values. dereference with *
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

// Storing diff types in a vector with enum
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];