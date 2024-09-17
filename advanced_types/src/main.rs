fn main() {
    println!("Hello, world!");
}

fn never_returns() -> ! {
    panic!()
}

// This Sized trait bound is implicit
fn generic_func<T: Sized>(t: T) {}

// This special syntax relaxes the restriction
fn generic_func_unsized<T: ?Sized>(t: &T) {}