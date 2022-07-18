// 6. 🌟🌟🌟 We can also implement methods for enums.


#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
   pub fn color(&self)-> String{
         match *self {
            Self::Red => "red".to_string(),
            Self::Yellow => "yellow".to_string(),
            Self::Green => "green".to_string(),
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}
