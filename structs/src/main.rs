struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = build_user(String::from("test@test.com"), String::from("test_user"));
    print_user(&user1);

    // use the struct update syntax to use values from existing struct
    let user2 = User {
        email: String::from("another@test.com"),
        username: String::from("anoter_user"),
        ..user1
    };
    print_user(&user2);
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

fn print_user(user: &User) {
    println!("{} ({})", user.username, user.email);
    println!("num sign ins: {}", user.sign_in_count);
    if user.active {
        println!("User is active.");
    } else {
        println!("User is not active.");
    }
    println!();
}