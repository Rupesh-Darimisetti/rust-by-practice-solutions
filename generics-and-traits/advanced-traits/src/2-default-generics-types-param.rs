/**
 * Default Generic Type Parameters
 * When we use generic type parameters, we can specify a default concrete type for the generic type. 
 * This eliminates the need for implementors of the trait to specify a concrete type if the default type works.
 */
// 2. 🌟🌟
use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// FILL in the blank in three ways: two of them use the default generic  parameters, the other one not.
// Notice that the implementation uses the associated type `Output`.
impl <T: Sub<Output = T>> Sub<Self>  for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 });

    println!("Success!");
}
