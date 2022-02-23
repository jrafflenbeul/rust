struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("rainerzufall"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("rainer@zufall.com");

    // not usable anymore due to move of user1 into user3
    // would be usable if only "active" and "sign_in_count"
    // are used for overwriting in user3, since only those
    // two implement the Copy trait instead of the Move trait
    /*let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("max@mustermann.de"),
        sign_in_count: user1.sign_in_count
    };*/

    let user3 = User {
        email: String::from("max@mustermann.de"),
        ..user1
    };

    // tuple structs - used for giving tuples a name
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like structs - behave similary to ()
    let subject = AlwaysEqual;
}
