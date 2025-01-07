use ca::UserManager;

fn main() {
    let user_manager = UserManager::new(
        "PetrineTest".to_string(),
        "Petrine@example.com".to_string(),
        "test".to_string(),
        "Petrine".to_string(),
        "1996-04-23".parse().expect("could not parse date"),
    );

    let user_manager = user_manager
        .login("Petrine@example.com".to_string(), "test".to_string())
        .expect("could not login");

    println!("Name: {}", user_manager.name());
}
