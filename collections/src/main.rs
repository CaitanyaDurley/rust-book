// collections contain multiple values
// unlike the built-in array and tuple types, the
// collections in the std lib are stored on the heap
// hence the amount of data does not need to be known
// at compile time and can change during runtime

mod hash_map;

fn main() {
    // a vector puts values next to each other in mem
    let v: Vec<i32> = Vec::new();
    // the vec! macro creates a vector from the values
    let mut v = vec![1, 2, 3];
    // append to a vec
    v.push(4);
    // get a reference to the elem at index 2
    let third = &v[2];
    // the method get returns an Option
    let third = v.get(2);
    match third {
        Some(third) => println!("third = {third}"),
        None => println!("No third elem"),
    }
    // indexing past the end of the vec will cause a panic
    // using get on an invalid index just returns None
    let first = &v[0];
    // first is a reference, and usual borrowing rules
    // hence apply
    // v.push(5); won't compile!
    println!("first = {first}");
    // why should an immutable reference to the first
    // elem prevent us appending to the vec?
    // because a vec's values are next to each other in
    // mem, appending might require allocating new mem
    // and copying the elems to the new space
    // in this case the ref would point to deallocated mem
    iterating(&mut v);
    untyped_vector();
    // when v goes out of scope and is dropped, all its
    // elems are also dropped - just like any other struct
    hash_map::main();
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// vectors can only store values of the same type, if you
// need a list of different types, encapsulate the different
// types in an enum
fn untyped_vector() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(12.2),
    ];
    // if you can't know the exhaustive set of types a
    // program will get at runtime, you can use a trait
    // object - covered later

}


fn iterating(v: &mut Vec<i32>) {
    for i in v {
        // i is a mutable reference to each elem of v
        // * dereferences the pointer so we change the value
        *i += 1;
        // attempting to insert/remove elems to v here
        // would cause a compile error
    }
}
