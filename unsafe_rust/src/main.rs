static HELLO_WORLD: &str = "hello world";
static mut COUNTER: u32 = 0;

fn main() {
    raw_pointers();
    safe_abstraction();
    static_vars();

}

fn raw_pointers() {
    let mut num = 5;
    // creating raw pointers is safe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = 0x012345usize as *const i32;
    // but dereferencing them is unsafe
    unsafe {
        println!("r1 is {}", *r1);
        dangerous();
    }
}

// an unsafe function indicates the function has requirements
// we need to uphold when calling it that Rust can't guarantee
// we've met. Calling it from within an unsafe block tells the
// compiler we're taking responsibility for upholding the
// function's contracts
unsafe fn dangerous() {}

fn safe_abstraction() {
    // the split_at_mut function from the std library is unsafe
    // code wrapped in a safe function.
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);
}

// our implementation to see why we need unsafe code
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut[i32], &mut[i32]) {
    let len = values.len();
    assert!(mid <= len);
    // Rust sees that we're taking two mutable references to the same slice
    // and refuses to compile. We know this is okay because we're borrowing
    // different parts of the slice, so we have to use unsafe code
    // (&mut values[..mid], &mut values[mid..])

    // get a raw pointer to the slice's buffer
    let ptr = values.as_mut_ptr();
    unsafe {(
        // recall a slice is just a pointer to some data and the length
        // of the slice. from_raw_parts_mut is an unsafe function because it
        // must trust that the raw pointer you provide is valid
        std::slice::from_raw_parts_mut(ptr, mid),
        std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )}
}

// an extern block lets you write the signatures of external functions in another
// language we want to call. The "C" part defines the application binary interface
// the external func uses. Functions in an extern block are always unsafe.
extern "C" {
    fn abs(input: i32) -> i32;
}

// you can also create your own interface for calling by other langs. Specify
// the ABI to use just before the fn keyword. You must tell the compiler not to
// mangle the function name
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("This code can be called from C");
}

fn static_vars() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// declare this trait to be unsafe
unsafe trait Foo {
    // methods here, at least one of which is unsafe
}

// we promise to uphold the invariants of Foo
unsafe impl Foo for i32 {
    // method implementations here
}