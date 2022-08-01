// HashMap
/**
 * Where vectors store values by an integer index, HashMaps store values by key. It is a hash map implemented with quadratic probing and SIMD lookup. 
 * By default, HashMap uses a hashing algorithm selected to provide resistance against HashDoS attacks.
 * The default hashing algorithm is currently SipHash 1-3, though this is subject to change at any point in the future. 
 * While its performance is very competitive for medium sized keys, other hashing algorithms will outperform it for small keys such as integers as well as 
 * large keys such as long strings, though those algorithms will typically not protect against attacks such as HashDoS.
 * The hash table implementation is a Rust port of Google’s SwissTable. The original C++ version of SwissTable can be found here, and this CppCon talk gives 
 * an overview of how the algorithm works.
 */
// FILL in the blanks and FIX the erros
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // get returns a Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // indexing return a value V
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score)
    }
}
