// passing a value to a function works the same way
// as assigning a value to a variable
fn main() {
    let s = String::from("hello");
    // value of s is moved into the func
    takes_ownership(s);
    // println!("{}", s); // doesn't compile!
    let x = 5;
    // i32 has the Copy trait so a copy is made
    makes_copy(x);
    println!("{}", x);
    let s2 = gives_ownership();
    // x goes out of scope so is popped off the stack
    // s goes out of scope but it was invalidated
    // so nothing happens
    // s2 is dropped
}

fn gives_ownership() -> String {
    let s = String::from("yours");
    s
    // s is moved to the calling func, so nothing
    // happens on out of scope
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // since the func has taken ownership,
    // after this scope ends the mem is freed
}
