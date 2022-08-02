/**
 * Convert by as
 * Rust provides no implicit type conversion(coercion) between primitive types. 
 * But explicit type conversions can be performed using the as keyword.
*/
// 1.🌟
 
// FIX the errors and FILL in the blank
// DON'T remove any code
fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char;
    let c2 = integer as char;

    assert_eq!(integer, 'b' as u8 - 1);

    println!("Success!")
}
