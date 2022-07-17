# For some cases, when matching enums, match is too heavy. We can use if let instead.
# 6. 🌟
```rs
fn main() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead 
    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);

            println!("Success!");
        }
        _ => {}
    };
}
```
# Solutions:
```rs
fn main() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead 
    if let Some(i) = o  {
            println!("This is a really long string and `{:?}`", i);

            println!("Success!");

        // _ => {};
    };
}
```
# 7. 🌟🌟
```rs
// Fill in the blank
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    __ {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}
```
# Solution:
```rs
// Fill in the blank
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a{
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}
```
# 8. 🌟🌟
```rs
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    // Remove the codes below, using `match` instead 
    if let Foo::Bar = a {
        println!("match foo::bar")
    } else if let Foo::Baz = a {
        println!("match foo::baz")
    } else {
        println!("match others")
    }
}
```
# Solution:
```rs
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    // Remove the codes below, using `match` instead 
    match a   {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others")
    }
}
```
