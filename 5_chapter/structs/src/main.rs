struct User {
    active: bool,
    name: String,
    email: String,
    age: u8,
}

fn main() {
    let mut user1 = User {
        active: true,
        name: String::from("Mike"),
        email: String::from("exmple@email.org"),
        age: 52,
    };

    user1.email = String::from("Another@email.com");
}
