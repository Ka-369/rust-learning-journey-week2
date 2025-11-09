/// Represents the state of a traffic signal.
#[derive(Debug)]
enum TrafficLight {
    Red,    // Stop signal
    Green,  // Go signal
    Orange, // Prepare to stop or go
}

/// Implementation of behavior for TrafficLight enum.
impl TrafficLight {
    /// Prints the action associated with each traffic light.
    fn take_action(&self) {
        match self {
            TrafficLight::Red => println!("Stop"),
            TrafficLight::Green => println!("Go"),
            TrafficLight::Orange => println!("Get Ready"),
        }
    }
}

/// Enum representing geometric shapes with associated dimensions.
#[derive(Debug)]
enum Shape {
    Circle(f64),         // Circle with given radius
    Square(f64),         // Square with given side length
    Rectangle(f64, f64), // Rectangle with height and width
}

/// Implementation for printing information about a shape.
impl Shape {
    /// Prints details about the shape.
    fn print_info(&self) {
        match self {
            Shape::Circle(radius) => println!("Circle with Radius : {}", radius),
            Shape::Square(side) => println!("Square of side length : {}", side),
            Shape::Rectangle(height, width) => println!("Rectangle of dimensions {} x {}", height, width),
        }
    }
}

fn main() {
    // Create a traffic light and print its action
    let signal = TrafficLight::Red;
    signal.take_action();

    // Create different shapes and print their info
    let c = Shape::Circle(3.0);
    let s = Shape::Square(6.0);
    let r = Shape::Rectangle(36.0, 9.0);

    c.print_info();
    s.print_info();
    r.print_info();
}
