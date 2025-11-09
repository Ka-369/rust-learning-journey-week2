// Define a trait representing a device that can be turned on.
trait PowerOn {
    fn turn_on(&self); // Method signature
}

// Define struct types for different devices.
struct Phone;
struct Laptop;

// Implement the trait for Phone.
impl PowerOn for Phone {
    fn turn_on(&self) {
        println!("Phone powers up");
    }
}

// Implement the trait for Laptop.
impl PowerOn for Laptop {
    fn turn_on(&self) {
        println!("Laptop boots!");
    }
}

// Generic function to start any PowerOn device.
fn device_starts<T: PowerOn>(device: &T) {
    device.turn_on();
}

fn main() {
    let my_phone = Phone;
    let my_laptop = Laptop;

    device_starts(&my_phone);
    device_starts(&my_laptop);
}
