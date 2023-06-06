fn main() {
    if let Some(token) = std::env::args().nth(1) {
        match auth::verify_user_token(&token) {
            Ok(user) => println!("{:?}", user),
            Err(e) => println!("ERROR: {}", e),
        }
    } else {
        println!("Usage: auth <token>");
    }
}
