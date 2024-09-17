// we focus on Strings here because the primitive
// data types seen earlier are a known size, so are
// stored on the stack, and can be quickly copied to
// make a new indep instance if another part of code
// needs the same val in a different scope

mod funcs;

fn main() {
    // A value remains valid until it goes out of scope
    {
        // s is a string literal - the value of the
        // string is hardcoded. Type = &str
        let s = "hi";
        // &str cannot be mutated (though it can be
        // overwritten with another literal if use mut)
    }
    // the scope is over, so s is no longer valid
    // The String type manages data allocated on the
    // heap and so can store an amount of text unkown
    // at compile time. Can create from a string literal
    // from requests mem from heap allocator
    let mut s = String::from("hi");
    // unlike &str, can mutate String
    s.push_str(" world");
    // the contents of a string literal are known
    // at compile time, so the text is hardcoded into
    // the executable. This makes string literals fast.
    // But only works because they're immutable - we
    // can't put a blob of memory into the binary for
    // each piece of text of unknown size and whose size
    // may grow during runtime.
    {
        let s = String::from("hello");
        // do stuff with s
    }
    // s goes out of scope, so Rust calls String::drop
    // to return the memory to the allocator
}

fn multiple_vars() {
    let x = 5;
    // y is bound to a copy of the value of x
    // both 5s are pushed onto the stack (known fixed size)
    let y = x;
    let s1 = String::from("hello");
    let s2 = s1;
    // a String is comprised of three parts:
    // the ptr
    // the length - how many bytes the contents are 
    // currently using
    // the capacity - how many bytes the String has
    // received for the allocator
    
    // These three parts are stored on the stack
    // when we assign s1 to s2, the String data on the
    // stack is copied, but the heap data itself is not

    // Now what happens when s1 and s2 go out of scope?
    // We would get a double free error! To ensure mem
    // safety, rust considers s1 as no longer valid and
    // so doesn't need to free anything when s1 goes
    // out of scope. This also means the following line
    // wouldn't compile
    // println!("{}", s1); // doesn't compile!

    // The line let s2 = s1 is known as a move: it
    // does a shallow copy and invalidates the first var

    // to do a deep copy of the heap data as well do
    let s1 = String::from("hi");
    let s2 = s1.clone();
    println!("{}", s1); // okay!

    // so how does Rust know to copy rather than move
    // a type like i32?
    // The Copy trait can be placed on types that are
    // stored on the stack. If a type has this trait,
    // vars are not moved but trivially copied, leaving
    // the orig. var still valid after assignment.

    // You cannot annotate a type with Copy if it (or any
    // of its parts) have the Drop trait.
}
