use serde::{Serialize, Deserialize};

// Define a struct User, enabling serialization and deserialization
#[derive(Serialize, Deserialize)]
struct User {
    username: String,   // User's name (snake_case for Rust convention)
    age: u32,           // User's age
}

fn main() {
    // Create a User instance with sample values
    let user = User {
        username: "Roy".to_string(),
        age: 30,
    };

    // Serialize the User struct to a JSON string
    let json_str = serde_json::to_string(&user).unwrap();
    println!("Serialized: {}", json_str); // Output serialized JSON

    // Deserialize the JSON string back to a User struct
    let deserialized_user: User = serde_json::from_str(&json_str).unwrap();
    // Output deserialized struct fields
    println!("Deserialized: username = {}, age = {}", deserialized_user.username, deserialized_user.age);
}
