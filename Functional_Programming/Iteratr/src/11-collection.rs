/* Fill in the blanks */
use std::collections::HashMap;
fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect();

    println!("{:?}",folks);
}
