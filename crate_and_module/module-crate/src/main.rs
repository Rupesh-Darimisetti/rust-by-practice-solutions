// in src/main.rs
// 5. 🌟🌟🌟 Now we will call a few library functions from the binary crate.

// FILL in the blank and FIX the errors
fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}
