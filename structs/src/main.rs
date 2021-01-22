struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = build_user(String::from("test@test.com"), String::from("test_user"));

    println!("{}", user1.username);

    // use the struct update syntax to use values from existing struct
    let user2 = User {
        email: String::from("another@test.com"),
        username: String::from("anoter_user"),
        ..user1
    };

    println!("{} ({})", user2.username, user2.email);
}

fn build_user(email: String, username: String) -> User {
    // using the field init shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}