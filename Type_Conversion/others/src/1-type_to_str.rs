/**
 * Convert any type to String
To convert any type to String, you can simply the ToString trait for that type. Rather than doing that directly, you should implement the fmt::Display trait which will automatically provides ToString and also allows you to print the type with println!.
*/
// 1. 🌟🌟
 
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!")
}
