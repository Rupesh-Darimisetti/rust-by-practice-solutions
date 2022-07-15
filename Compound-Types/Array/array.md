# The type of array is [T; Length], as you can see, array's length is part of their type signature. So their length must be known at compile time.
# For example, you cant initialize an array like below:
```rs
fn init_arr(n: i32) {
    let arr = [1; n];
}
```
# This will cause an error, because the compiler has no idea of the exact size of the array at compile time.

# 1. 🌟
```rs
fn main() {
    // Fill the blank with proper array type
    let arr: __ = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 4);

    println!("Success!");
}
```
# Solution:
```rs
fn main() {
    // Fill the blank with proper array type
    let arr: [i32;5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}
```
# 2. 🌟🌟
```rs
fn main() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == __);

    println!("Success!");
}
```
# Solution:
```rs
fn main() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let _arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}
```
# 3. 🌟 All elements in an array can be initialized to the same value at once.
```rs
fn main() {
    // Fill the blank
    let list: [i32; 100] = __ ;

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
```
# Solution:
```rs
fn main() {
    // Fill the blank
    let list: [i32; 100] = [1;100] ;

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
```
# 4. 🌟 All elements in an array must be of the same type
```rs
fn main() {
    // Fix the error
    let _arr = [1, 2, '3'];

    println!("Success!");
}
```
# Solution:
```rs
fn main() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("Success!");
}
```
# 5. 🌟 Indexing starts at 0.
```rs
fn main() {
    let arr = ['a', 'b', 'c'];
    
    let ele = arr[1]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}
```
# Solution:
```rs
fn main() {
    let arr = ['a', 'b', 'c'];
    
    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}
```
# 6. 🌟 Out of bounds indexing causes panic.
```rs
// Fix the error
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[2];

    println!("Success!");
}
```
# Solution:
```rs
// Fix the error
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1];

    println!("Success!");
}
```