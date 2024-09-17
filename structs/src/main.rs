// structs are like the fields of an object in OOP.

// struct derives the Debug trait from Rust
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// Note we used String rather than a string slice &str
// This means each instance of the struct to own its
// data, and for the data to be valid for as long as
// the struct is valid. A struct can store references
// but this requires lifetimes (cover later) to ensure
// the data referenced by the struct is valid for as
// long as the struct is

fn main() {
    let user1 = build_user(
        String::from("abc@def.com"),
        String::from("user1"),
    );
    // create a new struct but override some fields
    let user2 = User {
        email: String::from("def@ghi.com"),
        // struct update syntax must come last
        ..user1
    };
    // Note the above line MOVES the data from user1
    // This means the non-primitive types cannot
    // now be referenced
    // println!("{}", user1.username) would error
    println!("{}", user1.active); // okay
    println!("{}", user2.email);
    println!("{}", user2.email);
    let origin = Point(0, 0, 0);
    println!("{}", origin.0); // index like a tuple
    let subject = AlwaysEqual;
    // we might add behaviour to this type s.t. every
    // instance of AlwaysEqual is always equal to any
    // instance of any other type
    
    // println! macro can do many types of formatting
    // and by default the {} tell it to use Display
    // formatting - output intended for the end user
    // println!("user2 is {}", user2); doesn't implement fmt::Display
    
    // can use {:?} to tell println! we want the Debug
    // format
    println!("user2 is {:?}", user2);
    // and {:#?} for pretty print
    println!("user2 is {:#?}", user2);
    // can also use the dbg! macro. This macro takes
    // ownership of whatever you give it (println!
    // borrows) and returns ownership. Still needs
    // the debug trait
    let u = User {
        active: dbg!(true), // returns ownership so okay
        username: String::from("abc"),
        email: String::from("def"),
        sign_in_count: 1
    };
    dbg!(&u); // not capturing the return so pass reference
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// if your struct doesn't need field names, but you
// still want a custom types, you can use a tuple struct
struct Point(i32, i32, i32);

// if you don't need any fields, but still want a custom
// type (e.g. to implement a trait on some type but don't
// have any data to store in the type), can use a
// unit-like struct (so-called because behave like ())
struct AlwaysEqual;
