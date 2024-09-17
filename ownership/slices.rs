// A slice is a kind of reference (so does not have
// ownership). It lets you reference a contiguous
// sequence of elems in a collection (rather than the
// whole collection)

fn main() {
    let s = String::from("abc def");
    let first = first_word_v1(&s);
    s.clear(); // empties the string
    // first still has its value here, even though
    // without s it is meaningless
    let abc = &s[..3];
    let def = &s[4..];
    // these string slices reference a portion of the
    // string. The slice reference stores the start
    // position and length of the slice
    let s = String::from("abc def");
    let first = first_word_v2(&s);
    // s.clear(); would error!
}

// get first word in space separated string
fn first_word_v1(s: &String) -> usize {
    let bytes = s.as_bytes();
    // iter provides an iterator over the collection
    // and enumerate gives the (ix, &item) tup
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s
}

// Note that a string literal (&str) is just a slice
// pointing to a specific point in the binary. This is
// why string literals are immutable - they're just
// immutable references

// We can rewrite the signature of our func
// a slice can be passed, and a &String can also
// be passed due to deref coercions
fn first_word_v3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s
}