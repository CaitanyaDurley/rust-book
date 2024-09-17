fn main() {
    println!("Hello, world!");
    another_func(3);
}

// function bodies are a series of statements
// optionally ending in one expression
// statements perform some action but do not
// return a value. Expressions evaluate to a resultant value
fn another_func(x: i32) -> i64 {
    // this is a statment
    println!("x={x}");
    // so is this
    let x = 6;
    // which means you can't do this
    // let x = (let y = 6);
    // this is an expression
    6;
    // so is this block
    let y = {
        let x = 3;
        // a semicolon would make it a statement
        x + 1
    };
    println!("y={y}");
    // return keyword is optional - use to return early
    7
}
