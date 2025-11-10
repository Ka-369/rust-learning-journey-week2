use std::fs;

fn main() {
    // Read the file using unwrap (will panic if file not found or unreadable)
    let content_unwrap = fs::read_to_string("hello.txt").unwrap();

    // Print the content read with unwrap
    println!("File read with unwrap: {}", content_unwrap);

    // Read the file with proper error handling using match
    let content_result = fs::read_to_string("hello.txt");

    match content_result {
        Ok(text) => println!("File read with error handling: {}", text),
        Err(error) => println!("Failed to read file: {:?}", error),
    }
}
