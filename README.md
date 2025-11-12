# rust-learning-journey-week2

## Overview

This repository showcases my Week 2 progress in learning Rust, featuring hands-on 
implementations of key concepts like traits, error handling, enums, and JSON 
serialization. Each file demonstrates a different fundamental concept of the 
Rust programming language.


## 📑 Table of Contents
- [Overview](#overview)
- [Topics Covered](#-topics-covered)
- [Getting Started](#-getting-started)
- [What I Learned](#-what-i-learned)
- [Key Takeaways](#-key-takeaways)
- [License](#-license)
- [Contributing](#-contributing)
- [Contact](#-contact)


## 📚 Topics Covered

### 1. Error Handling (`Error-handling.rs`)
- Result type and error propagation
- Custom error types
- The `?` operator for concise error handling
- Handling recoverable errors gracefully

### 2. Traits (`Traits.rs`)
- Defining and implementing traits
- The `PowerOn` trait with implementations for various devices
- Trait methods and default implementations
- Using traits for shared behavior across types

### 3. Pattern Matching (`Pattern-matching.rs`)
- `match` expressions for control flow
- Interactive number selection game
- User input handling
- Exhaustive pattern matching

### 4. Enums (`Enums.rs`)
- Defining enums with different variants
- `TrafficLight` enum with color states
- `Shape` enum with associated data
- Methods on enums
- Pattern matching with enums

### 5. JSON Serialization (`json-serde.rs`)
- Working with the `serde` crate for serialization and deserialization
- Using `Serialize` and `Deserialize` traits
- Converting Rust structs to JSON strings
- Parsing JSON back into Rust structs
- Practical example with a User struct

## 🚀 Getting Started

### Prerequisites
- Rust installed (1.70.0 or later recommended)
- Cargo package manager


## 📁 Folder Structure
rust-learning-journey-week2/
│
├── Cargo.toml          
├── Enums.rs
├── Traits.rs
├── Pattern-matching.rs
├── Error-handling.rs
├── json-serde.rs
├── LICENSE
└── README.md


### Running the Programs

Each file can be compiled and run individually:

```bash
# Compile a specific file
rustc Enums.rs
./Enums

# Or run with cargo
rustc Pattern-matching.rs && ./Pattern-matching
```

### For json-serde.rs, use cargo (includes dependencies)

cargo run --bin json-serde

**Dependencies:**
- `serde = { version = "1.0", features = ["derive"] }`
- `serde_json = "1.0"`


## 📖 What I Learned

- **Error Handling**: Understanding how Rust's Result and Option types provide safe error handling without exceptions
- **Traits**: How to define shared behavior and create flexible, reusable code
- **Pattern Matching**: The power of Rust's pattern matching for writing expressive and safe code
- **Enums**: Creating custom types that can represent multiple variants with different associated data
- **JSON Serialization**: Working with serde and serde_json for seamless conversion between Rust structs and JSON format

## 🎯 Key Takeaways

1. Rust's type system encourages explicit error handling
2. Traits enable polymorphism without inheritance
3. Pattern matching provides exhaustive case handling at compile time
4. Enums are more powerful than in many other languages
5. Serde provides powerful derive macros for automatic serialization support

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

This is a personal learning repository, but suggestions and feedback are welcome! Feel free to open an issue if you spot any improvements.

## 🌿 Git Branching

- **main** → stable and reviewed code  
- **dev** → work-in-progress code for ongoing concepts  
- **feature/*** → separate branches for each topic (e.g., `feature/traits`, `feature/enums`)  
- Use pull requests to merge new learnings into `main` after testing  

## 📬 Contact
Feel free to reach out if you have questions or want to discuss Rust concepts!


---

**Happy Learning! 🦀**
