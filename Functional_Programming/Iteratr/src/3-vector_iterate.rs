/* Fill the blanks and fix the errors.
Using two ways if possible */
fn main() {
    let v1 = vec![1, 2];
    
    let mut v1_iter = v1.into_iter();
    
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), None);
}
