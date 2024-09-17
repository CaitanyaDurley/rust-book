// basic enum
enum IpAddrKind {
    V4,
    V6
}

// enum can also store data
// this is more concise than a struct storing an
// IpAddrKind and the actual ip string
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

// can define methods on enums using impl
impl IpAddr {
    fn show(&self) {
        dbg!(self);
    }
}

// rust does not have nulls - if you try to use a
// null value as a not-null value you'll get an
// error. This is an extremely common bug.
// Instead rust has an enum to say whether a value
// is present or absent
// enum Option<T> {
//  None,
//  Some(T)
// }
// Option is included in the prelude (you don't need to
// bring it into scope explicitly). It's variants None
// and Some are also in the prelude so you don't have
// to do Option::Some(value)

enum Coin {
    Penny,
    Nickel,
    Dime,
    // quarters have a US State on the back
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    // recall a match matches values to patterns
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // here we're binding state to the value
        // in the enum variant!
        Coin::Quarter(state) => {
            println!("Quarter state: {:?}", state);
            25
        },
    }
}

// matching Option types
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// using a catch all pattern
fn dice_roll(roll: i8) {
    match roll {
        3 => println!("{}", 3),
        other => println!("not 3: {}", other),
        // can use _ if you don't need the value
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr::V4(String::from("127.0.0.1"));
    home.show();
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // Option is better than null because T and
    // Option<T> are of different types, so the type
    // checker will not let you treat Option<T> as
    // if it were T
    // x + y doesn't compile!
    value_in_cents(Coin::Quarter(UsState::Alabama));
    let config_max = Some(3u8);
    // consider the case where you only need to do
    // something with the value in one case. Could do:
    match config_max {
        Some(max) => println!("max is {max}"),
        _ => (), // this is annoying boilerplate
    }
    // Instead use if let
    if let Some(max) = config_max {
        println!("max is {max}")
    } else {
        println!("Can also provide else block")
    }
}
