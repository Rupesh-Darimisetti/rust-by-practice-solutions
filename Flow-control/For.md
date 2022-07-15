# 3. 🌟 The for in construct can be used to iterate through an Iterator, e.g a range a..b.
```rs
fn main() {
    for n in 1..=100 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 
```
# Solution:
```rs
fn main() {
    for n in 1..100 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 
```
# 4. 🌟🌟
```rs
// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with name...
    }
    
    println!("{:?}", numbers);
} 
```
# Solution:
```rs 
// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with name...
    }
    
    println!("{:?}", numbers);
} 
```
# 5. 🌟
```rs
fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.__ {
        println!("The {}th element is {}",i+1,v);
    }
}
```
# Solution:
```rs
fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {
        println!("The {}th element is {}",i+1,v);
    }
}
```