fn main() {
    
    struct User {
        username: String,
        email: String,
        password: String,
    };

    let user1 = User{
        email: String::from("rust@udemy.com"),
        username: String::from("rustUdemy"),
        password: String::from("******"),
    };

    println!("E-mail = {}", user1.email);
}
