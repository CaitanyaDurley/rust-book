use std::thread;

use closures::{ShirtColour, Inventory};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };
    let preference1 = Some(ShirtColour::Red);
    let giveawawy1 = store.giveaway(preference1);
    println!("User with preference {:?} gets {:?}", preference1, giveawawy1);
    let preference2 = None;
    let giveawawy2 = store.giveaway(preference2);
    println!("User with preference {:?} gets {:?}", preference2, giveawawy2);
    more_closures();
}

fn more_closures() {
    let mut list = vec![1, 2, 3];
    println!("{:?}", list);
    let immut_borrow = || println!("From closure: {:?}", list);
    immut_borrow();
    println!("{:?}", list);
    // because the closure's reference to list is part of the closure,
    // if we modify list this actually modifies the closure too!
    // hence we must define the closure as mutable itself
    let mut mut_borrow = || list.push(4);
    mut_borrow();
    println!("{:?}", list);
    // to force the closure to take ownership of values in the environ use move
    // mostly useful when passing a closure to a new thread and you want the
    // data to be owned by the new thread - you have to move the values in case
    // the main thread finished first and dropped them
    thread::spawn(move || println!("From thread: {:?}", list))
        // wait for thread to finish
        .join()
        .unwrap();
}
