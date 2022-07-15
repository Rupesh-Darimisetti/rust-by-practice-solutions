# 1. 🌟 Elements in a tuple can have different types. Tuple's type signature is (T1, T2, ...), where T1, T2 are the types of tuple's members.
```rs
fn main() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, __, i64, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}
```
# Solution:
```rs
fn main() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}
```
# 2. 🌟 Members can be extracted from the tuple using indexing.
```rs
// Make it work
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "sunface");

    println!("Success!");
}
```
# Solution:
```rs
// Make it work
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success!");
}
```
# 3. 🌟 Long tuples cannot be printed
```rs
// Fix the error
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
}
```
# Solution:
```rs
// Fix the error
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}
```
# 4. 🌟 Destructuring tuple with pattern.
```rs
fn main() {
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let __ = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}
```
# Solution:
```rs
fn main() {
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}
```
# 5. 🌟🌟 Destructure assignments.
```rs
fn main() {
    let (x, y, z);

    // Fill the blank
    __ = (1, 2, 3);
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}
```
# Solution:
```rs
fn main() {
    let (x, y, z);

    // Fill the blank
    (y, z, x) = (1, 2, 3);
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}
```
# 6. 🌟🌟 Tuples can be used as function arguments and return values
```rs
fn main() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply(__);

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
```
# Solution:
```rs
fn main() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
```