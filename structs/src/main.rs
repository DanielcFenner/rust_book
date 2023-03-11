struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("dcfenner@gmail.com"),
        active: true,
        username: String::from("fenner"),
        sign_in_count: 54,
    };

    let user2 = build_user("steve@gmail.com".to_string(), "steve".to_string());

    // update struct syntax similar to spread operator

    let user2 = User {
        email: "steven_johnson@gmail.com".to_string(),
        ..user2
    };

    println!("{}", &user2.email);
    println!("{}", &user2.email);
}
// field init shorthand, aslong as we have same param names as struct
// we dont need to type email: email
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
