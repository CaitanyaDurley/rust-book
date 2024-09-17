fn main() {
    // Could let every func take ownership of the
    // params and then return them back
    let s1 = String::from("hi");
    let (s2, len) = manual_borrowing(s1);
    println!("length of {s2} is {len}");
    // Instead we can provide a reference to the
    // String. A reference is like a pointer in that
    // it's an address we can follow to access some
    // data. Unlike a pointer, a reference
    // is guaranteed to point to a valid value of a
    // given type for the lifetime of that reference
    let s1 = String::from("hi");
    // here &s1 is a reference to s1 - a pointer
    // to s1 itself (on the stack?) not the location
    // in the heap that s1 has a pointer to
    // creating a reference is called borrowing
    let len = calculate_length(&s1);
    println!("length of {s1} is {len}");
    let mut s1 = String::from("hi");
    change(&mut s1);
    println!("{s1}");
    // If you have a mutable reference to a value,
    // you can have no other references to that value
    let r1 = &mut s1;
    // let r2 = &s1; // error!
    // let r2 = &mut s1; // error!
    // This restriction ensures Rust can prevent
    // data races at compile time.
    {
        // there's no problem having multiple mutable
        // references in a different scope because
        // they're not simultaneous
        let r2 = &s1;
        // r2 goes out of scope
    }
}

fn manual_borrowing(s: String) -> (String, usize) {
    // takes ownership of s1
    let length = s.len();
    (s, length)
    // gives ownserhip of s
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope, but because we don't
    // have ownership so we do nothing. We couldn't do
    // anything regardless because our reference
    // doesn't have the Drop trait!
}

// you can't use a reference to modify the value
// fn change(s: &String) {
fn change(s: &mut String) {
    // but you can use a mutable reference
    s.push_str(" world!");
}

fn reference_scope() {
    // A reference's scope ends the last time that ref
    // is used. Hence this code is valid:
    let mut s = String::from("hi");
    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");
    // scopes of r1, r2 end here
    let r3 = &mut s; // okay because scopes don't overlap
    println!("{r3}");
}

// A dangling pointer is a pointer that references
// a loc in mem that has been freed
// below doesn't compile:
// fn dangling_reference() -> &String {
//     let s = String::from("hi");
//     &s
// }
// Rust ensures the data doesn't go out of scope
// before the reference does
