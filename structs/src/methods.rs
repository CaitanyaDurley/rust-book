// methods are funcs defined within the context of
// a struct, enum or trait object

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block for Rectangle: associated
// functions all go in here
impl Rectangle {
    // first param of a method must be called self
    // Within an impl block, the type Self is an alias
    // for the relevant type. Can be self or &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // can clash a method name with a field name
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // this is NOT a method because it doesn't have
    // self as its first param. It is an associated
    // function - and in particular a constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


// if desired, you can have multiple impl blocks on a
// struct, there's no reason here but it is valid
// syntax and is useful for generic types and traits

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
        };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // if brackets, rust assumes we want the method
    if rect1.width() {
        // if no brackets, it's the field
        println!("width={}", rect1.width);
    }
    // why is it not necessary to pass a ref like:
    (&rect1).width();
    // because Rust has auto referencing and deref
    // in this case an immutable ref is passed because
    // the method expects &self. Rust will also deref
    // a pointer if the method expects self

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
        let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // to call the associated func, use :: because the
    // func is namespaced by the struct
    let sq = Rectangle::square(3);
    dbg!(sq);
}
