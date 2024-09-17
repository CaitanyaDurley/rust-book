use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;


fn main() {
    let b = Box::new(5);
    // the i32 value is on the heap
    println!("b = {b}");
    let cons_list = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Nil)
        ))
    );
    let x = 5;
    let y = Box::new(x);
    let z = MyBox::new(x);
    // can deref the smart pointer like a reference
    assert_eq!(x, *y);
    assert_eq!(x, *z);
    
    let deref_coercion = MyBox::new("Rust".to_string());
    // &MyBox is deref'd to &String which is deref'd to &str
    hello(&deref_coercion);

    dropping_early();
    reference_counting();
    reference_cycle();
    non_ownership_relationships();

    // drop is called on y and z
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

// Create a recursive type (Cons list)
// using Box to ensure a known size
enum List {
    Cons(i32, Box<Self>),
    Nil,
}

// We'll create our own version of Box which we can implement
// Deref on to make it a smart pointer (even though its data
// won't necessarily be stored on the heap)
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

// implement the Deref trait
impl<T> Deref for MyBox<T> {
    // Target is an associated type for the Deref trait
    type Target = T;

    // The compiler can deref any & reference.
    // By implementing the Deref trait, the compiler can
    // get a & reference that it knows how to dereference
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    // drop must take a mutable reference to Self
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}

// Rust doesn't let you call drop explicitly on your smart pointer
// if you need to drop the value early (e.g. to free a lock so that
// something else can acquire the lock), you must use std::mem::drop
// this function is in the prelude, and actually does nothing!
// Because it takes ownership of the value, the value is dropped
// automatically!
fn dropping_early() {
    let my_box = MyBox::new(123);
    // my_box.drop();   won't compile!
    drop(my_box);
    println!("Dropped MyBox early");
}

enum List2 {
    Cons(i32, Rc<Self>),
    Nil,
}

fn reference_counting() {
    let a = List2::Cons(
        5,
        Rc::new(List2::Cons(
            10,
            Rc::new(List2::Nil),
        )),
    );
    // store a on the heap and count references to it
    let rc_a = Rc::new(a);
    // convention to use Rc::clone(&rc_a) rather than rc_a.clone()
    let b = List2::Cons(3, Rc::clone(&rc_a));
    println!("Count after creating b: {}", Rc::strong_count(&rc_a));
    {
        let c = List2::Cons(4, Rc::clone(&rc_a));
        println!("Count after creating c: {}", Rc::strong_count(&rc_a));
    }
    println!("Count after dropping c: {}", Rc::strong_count(&rc_a));
    // once a goes out of scope, rc_a has a strong_count of 0, and its
    // implementation of Drop will cleanup the data from the heap
}

// Instead of multiple owners, we could have List hold a
// reference to the next elem. But then we need lifetime
// parameters to specify that every elem in the list lives
// at least as long as the whole list
enum List3<'a> {
    Cons(i32, &'a Self),
    Nil,
}

#[derive(Debug)]
enum List4 {
    Cons(i32, RefCell<Rc<Self>>),
    Nil,
}

impl List4 {
    fn tail(&self) -> Option<&RefCell<Rc<Self>>> {
        match self {
            Self::Cons(_, item) => Some(item),
            Self::Nil => None,
        }
    }
}

fn reference_cycle() {
    let a = Rc::new(List4::Cons(
        5,
        RefCell::new(Rc::new(List4::Nil)),
    ));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    // b points to a
    let b = Rc::new(List4::Cons(
        10,
        RefCell::new(Rc::clone(&a)),
    ));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        // modify a to point to b (deref the mutable reference)
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // we now have a reference cycle, stack overflow!
    // println!("a next item = {:?}", a.tail());
}

#[derive(Debug)]
struct Node {
    value: i32,
    // have a weak reference to our parent (if we had a strong reference
    // then we'd have a refrence cycle and could never drop anything)
    parent: RefCell<Weak<Self>>,
    children: RefCell<Vec<Rc<Self>>>,
}

fn non_ownership_relationships() {
    let leaf = Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    };
    let leaf_rc = Rc::new(leaf);
    let branch = Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf_rc)]),
    };
    let branch_rc = Rc::new(branch);
    *leaf_rc.parent.borrow_mut() = Rc::downgrade(&branch_rc);
    println!("leaf parent = {:?}", leaf_rc.parent.borrow().upgrade());
}