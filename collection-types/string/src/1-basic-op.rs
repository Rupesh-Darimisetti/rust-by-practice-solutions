/**
 * String
 * std::string::String is a UTF-8 encoded, growable string. 
 * It is the most common string type we used in daily dev, 
 * it also has ownership over the string contents.
*/
// Basic operations
//  1. 🌟🌟

// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Dont't add/remove any code line
fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str(&"world".to_string());
    s.push('!');

    move_ownership(s.clone());
    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
