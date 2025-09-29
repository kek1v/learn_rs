struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u8,
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        name: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, name: String) -> User{ // with ShortHand field init
    User{
        active: true,
        name, // field and parameters have the same name
        email,
        age: 52,
    }
}


fn main() {
    let mut user1 = User {
        active: true,
        name: String::from("Mike"),
        email: String::from("exmple@email.org"),
        sign_in_count: 52,
    };

    user1.email = String::from("Another@email.com");

    let mut user2 = build_user(String::from("u2Email@mail.com"), String::from("user2"));
}
