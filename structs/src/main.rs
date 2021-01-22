struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = build_user(String::from("test@test.com"), String::from("test_user"));

    println!("{}", user1.username);
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