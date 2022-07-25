/**
 * Fully Qualified Syntax
 * Nothing in Rust prevents a trait from having a method with the same name as another 
 * trait’s method, nor does Rust prevent you from implementing both traits on one type. 
 * It’s also possible to implement a method directly on the type with the same name as methods from traits.
 * When calling methods with the same name, we have to use Fully Qualified Syntax.
 */
// 3. 🌟🌟
trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

fn main() {
    let person = Human;

    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
    assert_eq!(Wizard::fly(&person), "Up!");

    assert_eq!(person.fly(), "*waving arms furiously*");

    println!("Success!");
}
