/**
 * UTF-8 & Indexing
 * Strings are always valid UTF-8. This has a few implications:
 * 
 * the first of which is that if you need a non-UTF-8 string, consider OsString. It is similar, but without the UTF-8 constraint.
 * The second implication is that you cannot index into a String
 * Indexing is intended to be a constant-time operation, but UTF-8 encoding does not allow us to do this. 
 * Furthermore, it’s not clear what sort of thing the index should return: a byte, a codepoint, or a grapheme cluster. 
 * The bytes and chars methods return iterators over the first two, respectively.
 */
// 4. 🌟🌟🌟 You can't use index to access a char in a string, but you can use slice &s1[start..end].

// FILL in the blank and FIX errors
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];// tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");
    
    // iterate all chars in s
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}
