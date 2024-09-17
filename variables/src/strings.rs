// a String is a wrapper around a Vec of bytes
// with some extra guarantees, restrictions
// and capabilties. Hence many ops available
// on Vec<T> are also available on a String
// strnigs are UTF-8 encoded
pub fn main() {
    let s = String::new();
    // same as String::from("initial contents")
    let mut s = "initial contents".to_string();
    // push_str takes a string slice so we don't have
    // to take ownership of the parameter
    s.push_str("abc");
    // push takes a single character
    s.push('l');
    let s2 = " abc".to_string();
    // concatenation
    let s3 = s + &s2;
    // note s1 has been moved and can no longer be used
    // + uses the add method and looks like
    // fn add(self, s: &str) -> String
    // which is why the second param must be a ref
    // the compiler coerces the &String param into &str
    // modifies self and returns ownership - this is more
    // efficient than copying s
    
    // concatenating multiple strings with + is ugly
    // s1 + "-" + &s2 + "-" + &s3
    // instead use the format! macro
    let s = format!("{s2}-{s3}");
    // format uses references so s2, s3 still valid
    indexing_strings();
    operating_strings();
}

fn operating_strings() {
    // instead of indexing, be explict about whether
    // you want chars or bytes
    let s = String::from("hello");
    for c in s.chars() {
        println!("{c}");
    }
    for b in s.bytes() {
        // remember valid unicode scalar values may be
        // more than one byte!
        println!("{b}");
    }
}

fn indexing_strings() {
    let s1 = String::from("hello");
    // let h = s1[0]; won't compile!
    // String does not implement the Index<integer>
    // trait - you can't access a char via its index
    // the reason is that in UTF-8, a letter may take
    // 1 or 2 bytes to store (ascii vs unicode). It's
    // impossible to know whether the user wants the
    // nth letter or the nth byte
    // in Hindi, a unicode scalar value might be a letter
    // or a diacritic (an accent), here it makes even
    // less sense, since a human's 'letter' refers to a
    // letter and its diacritic
    
    // this gets the first 4 bytes of the string
    let s2 = &s1[0..4];
    // if you tried to slice not on a char boundary
    // rust will panic at runtime
}
