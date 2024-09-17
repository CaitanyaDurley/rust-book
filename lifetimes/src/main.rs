fn main() {
    dangling_reference();

    let s1 = "abcd".to_string();
    let s2 = "xyz";
    // pass in string slices, not Strings, so don't lose ownership
    let result = longest(s1.as_str(), s2);
    println!("Longest = {}", result);

    let novel = String::from("Sentence 1. Sentence 2");
    let first_sentence = novel.split('.').next().expect("No .");
    let excerpt = Excerpt {
        part: first_sentence,
    };
    
    static_lifetimes();
}

fn dangling_reference() {
    // recall rust has no nulls, trying to use r before assigning
    // it a value will result in a compile-time error
    let r;
    {
        let x = 5;
        r = &x;
        // x goes out of scope and is dropped
    }
    // compile time error: x does not live long enough
    // println!("r: {}", r);
}

// dosen't compile - need a lifetime parameter since
// fn longest(x: &str, y: &str) -> &str {
// the return type is a borrowed value - borrowed from x or y?
// in fact we don't know - it depends on their lengths
// annotate with a generic lifetime - x and y both live at least
// as long as 'a, and the return value also will live at least
// as long as 'a. In other words, the return value lives at least
// as long as the smaller lifetime of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// As with generic types, it works on structs
struct Excerpt<'a> {
    // part must live as long as Excerpt
    part: &'a str
}

fn static_lifetimes() {
    // The lifetime 'static, means the reference CAN live for the
    // entiure duration of the program. E.g. string literals
    let s:&'static str = "I am stored in the binary";
}