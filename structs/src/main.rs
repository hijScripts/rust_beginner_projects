struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     let mut user2 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user2.email = String::from("anotheremail@example.com");

//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };

//     // shorthand struct update syntax
//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// // as long as you use params with same name as field, no need to repeat field: field
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // I can no longer access user1.username as ownership has moved to user2.
    let user2 = User {
        email: String::from("hey"),
        ..user1
    };

    // println!("{0} {1} {2}", user2.email, user1.email, user1.username) // this will error
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin // returns x, y, and z
}

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}