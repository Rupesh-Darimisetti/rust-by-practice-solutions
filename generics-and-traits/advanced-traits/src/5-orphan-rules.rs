/**
 * Orphan Rules
 * We can’t implement external traits on external types. 
 * For example, we can’t implement the Display trait on Vec<T> within our own crate, 
 * because Display and Vec<T> are defined in the standard library and aren’t local to our crate.
 * 
 * This restriction is often called the orphan rule, so named because the parent type is not present. 
 * This rule ensures that other people’s code can’t break your code and vice versa.
 * It’s possible to get around this restriction using the newtype pattern, which 
 * involves creating a new type in a tuple struct.
 */

 use std::fmt;

// DEFINE a newtype `Pretty` here
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
