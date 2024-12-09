struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("someone@example.com"),
        sign_in_count: 0,
    };

    user1.email = String::from("alex@gmail.com");
    println!("User1 email: {}", user1.email);

    let user2 = build_user(String::from("someone@example.com"), String::from("user2"));
    println!("User2 email: {}", user2.email);

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("User3 email: {}", user3.email);

    let user4 = User {
        active: true,
        username: String::from("user3"),
        email: String::from("other@example.com"),
        ..user1
    };
    println!("User4 email: {}", user4.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);
}
