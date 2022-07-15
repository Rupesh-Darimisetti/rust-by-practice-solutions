# 7. 🌟 Use break to break the loop.
```rs
// Fill in the blank
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n == 66 {
           __
       }
       n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
```
# Solution:
```rs
// Fill in the blank
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n == 66 {
           break;
       }
       n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
```
# 8. 🌟🌟 continue will skip over the remaining code in current iteration and go to the next iteration.
```rs
// Fill in the blanks
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           __;
       }
       
       __
    }

    assert_eq!(n, 66);

    println!("Success!");
}
```
# Solution:
```rs
// Fill in the blanks
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           continue;
       }
       
       break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}
```