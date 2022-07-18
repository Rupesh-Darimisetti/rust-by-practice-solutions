/**
 * 4. 🌟🌟 All functions defined within an impl block are called associated functions 
 * because they’re associated with the type named after the impl. We can define associated 
 * functions that don’t have self as their first parameter (and thus are not methods) because 
 * they don’t need an instance of the type to work with.
 */

 
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // 1. Implement an assotiated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
    pub fn new() -> Self {
        Self{
         color : "red".to_string()
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}
