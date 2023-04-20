fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@test.com"),
        sign_in_count: 1,
    };

    println!(
        "user1 details active: {}, username: {}, email: {}, sign_in_count: {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );
    user1.username = String::from("other_username");
    println!("new username: {}", user1.username);

    let user2 = build_user(String::from("user2"), String::from("user2@test.com"));
    println!(
        "user2 details active: {}, username: {}, email: {}, sign_in_count: {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    let user3 = User {
        username: String::from("user3"),
        // email: user1.email.clone(), // NOTE: this will make access to user1.email OK later if desired.
        ..user1 // similar to spreading in JS but with opposite sequence
    };
    println!(
        "user3 details active: {}, username: {}, email: {}, sign_in_count: {}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );

    // // NOTE: below will fail as user3 has borrowed email from user1
    // println!(
    //     "user1 details active: {}, username: {}, email: {}, sign_in_count: {}",
    //     user1.active, user1.username, user1.email, user1.sign_in_count
    // );

    // ----------- Tuple Structs -----------
    let black = Color(0, 0, 0);
    let origin = Point(0., 0., 0.);
    println!(
        "black={},{},{}; origin={},{},{}",
        black.0, black.1, black.2, origin.0, origin.1, origin.2
    );

    // ----------- Unit-like Structs -----------
    let object1 = AlwaysEqual;
    // let object2 = AlwaysEqual;
    // assert_eq!(object1, object2);

    // let user_without_lifetime = UserWithoutLifetime {
    //     active: true,
    //     username: "user4",
    //     email: "user4@test.com",
    //     sign_in_count: 1,
    // };
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // short hand here instead `username: username`
        email,    // short hand syntax
        sign_in_count: 1,
    }
}
struct Color(i32, i32, i32);
struct Point(f64, f64, f64);

struct AlwaysEqual;

// // This will fail without 'lifetime' keyword
// struct UserWithoutLifetime {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
