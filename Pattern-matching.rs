use std::io;

fn main() {
    loop {
        // Prompt the user for input
        println!("Select a number from 1 to 3:");

        // Read user input from stdin
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim whitespace and attempt to parse input as i32
        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // Invalid input (not a number), prompt again
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Respond based on the parsed number
        match number {
            1 | 2 => {
                // Both 1 and 2 result in a loss
                println!("You lose");
                break;
            }
            3 => {
                // 3 is the winning number
                println!("You won!");
                break;
            }
            _ => {
                // Any other number or out-of-range input is rejected
                println!("Number not in range (1-3), try again.");
            }
        }
    }
}
