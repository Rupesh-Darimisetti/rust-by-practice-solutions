/**
 * Static and Dynamic dispatch
 * When we use trait bounds on generics, the compiler generates nongeneric implementations of functions 
 * and methods for each concrete type that we use in place of a generic type parameter. 
 * The code that results from monomorphization is doing static dispatch, which is when the compiler 
 * knows what method you’re calling at compile time.
 * When we use trait objects, Rust must use dynamic dispatch. 
 * The compiler doesn’t know all the types that might be used with the code that is using trait objects,
 * so it doesn’t know which method implemented on which type to call. Instead, at runtime, Rust uses 
 * the pointers inside the trait object to know which method to call. There is a runtime cost when this
 * lookup happens that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler
 * from choosing to inline a method’s code, which in turn prevents some optimizations.
 * However, we do get extra flexibility when using dynamic dispatch.
 */
// 4. 🌟🌟

trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// IMPLEMENT below with generics.
fn static_dispatch<T: Foo>(x: T){
    x.method();
}

// Implement below with trait objects.
fn dynamic_dispatch(x: &dyn Foo){
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!");
}
