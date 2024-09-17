use std::io;

// constants are always immutable
const SECONDS_IN_AN_HOUR: u32 = 60;

// just tell rust-analyzer to check strings.rs
mod strings;

fn main() {
    let x = 5;
    // this is shadowing - the compiler sees
    // this new value when referencing x
    let x = x + 1;
    {
        // unlike let mut, can change types
        let x = "abc";
        println!("Block scope {x}");
        // block shadowing ends here
    }
    println!("Outer scope {x}");
    57u8; // number literals allow a suffix
    5_000_000; // underscores visual only
    // when compiling in debug mode, rust checks for
    // integer overflows and will panic if it happens
    // when compiling in release mode, there are no such
    // checks and if an overflow occurs then complement
    // wrapping occurs (i.e. max + 1 = min)
    // there are families of methods for handling
    // the possibility of overflows
    // floats are 64bit by default
    let _ = 56.7 / 32.2f32; // float division
    let _ = -5 / 3; // integer division = -1
    let _ = 45 % 2; // remainder/mod
    let _ = true;
    let c = 'Z'; // a char = 4 byte unicode
    // tuple, fixed length, different types possible
    let tup = (500, 6.4, true);
    // a tuple is a single compound element, to get values
    // out of it can use destructuring
    let (x, y, z) = tup;
    // or use a dot followed by index
    let x = tup.0;
    println!("y={y}");
    // The empty tuple is called the unit, it is its own type
    // expressions implicitly return the unit value if they
    // don't return any other value
    let z = ();
    // an array is of fixed length and all elems must have same type
    let a = [1,2,3,4,5];
    // arrays are allocated on the stack (not the heap)
    // a vector in the std lib can grow/shrink
    // init to 10 'a's
    let b:[char; 10] = ['a'; 10];
    let elem = a[0];
    println!("0 -> {elem}");
    index_error();
    strings::main();
}

fn index_error() {
    let a = [1, 2, 3];
    println!("Enter an index");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("enter a number");
    // in many low-level langs, there is no check for out of bounds
    // resulting in invalid mem being accessed. Rust checks this for
    // you and panics if invalid index.
    let element = a[index];
    println!("Value at index {index} is {element}");
}
