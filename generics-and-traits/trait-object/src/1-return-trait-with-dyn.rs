/**
 * Returning Traits with dyn
 * The Rust compiler needs to know how much space a function's return type requires. 
 * Because the different implementations of a trait probably uses different amounts of memory, 
 * functions need to either return a concrete type or the same type when using impl Trait, or 
 * return a trait object with dyn.
 */
// 1. 🌟🌟🌟
 trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn main() {
    // FILL in the blank.
    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}   

// IMPLEMENT this function.
fn hatch_a_bird(spec: u8) -> Box<dyn Bird>{
    if spec == 1 {
        Box::new(Swan{})
    } else {
        Box::new(Duck{})
    }
}

