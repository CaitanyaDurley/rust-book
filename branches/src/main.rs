fn main() {
    println!("Hello, world!");
    let number = 5;
    // condition must be a bool
    // can't do if number then ...
    if number < 5 {
        println!("n < 5");
    } else if number > 5 {
        println!("x > 5");
    } else {
        println!("x=5");
    }
    // if blocks are expressions
    let x = if 5>0 {5} else {6};
    println!("x={x}");
}
