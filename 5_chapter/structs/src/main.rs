// we cant use a &str type because this type have to know a lifetimes

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
        sign_in_count: 52,
    }
}

// tuple structs
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);


//unit-like struct
struct AlwaysEqual;


fn main() {
    let mut user1 = User {
        active: true,
        name: String::from("Mike"),
        email: String::from("exmple@email.org"),
        sign_in_count: 52,
    };
    user1.email = String::from("Another@email.com");
    let mut user2 = build_user(String::from("u2Email@mail.com"), String::from("user2"));



    // creating a new user instance using a def methods, without struct update syntax
    let mut user2 = User {
        active: true,
        name: user1.name,
        email: String::from("anotherOne@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    
    //and the same, but using struct update syntax
    let mut user3 = User {
        email: String::from("anotherOne@example.com"),
        ..user2
    };


    let black = Color(0,0,0);
    let origin = Point(0,0,0);


    let subject = AlwaysEqual;

}
