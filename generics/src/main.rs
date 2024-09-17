// there is no runtime cost for generic type params
// because at compile time rust turns generic code into
// specific code for each concrete type the generic
// code is called with.

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['a', 'b', 'c'];
    println!("largest = {}", largest(&numbers));
    println!("largest = {}", largest(&chars));
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// this is exactly the same code
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for c in list {
        if c > largest {
            largest = c;
        }
    }
    largest
}

// Have to restrict T to types implementing the
// PartialOrd trait
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// also works on structs. Can use more than one generic
// type to not restrict both x and y to the same type
struct Point<T, U> {
    x: T,
    y: U
}

enum Option<T> {
    Some(T),
    None,
}

// and on implementations; you have to
// declare generic types in the impl
// here the method x is only implemented
// on Points where y is a f32
impl<T> Point<T, f32> {
    fn x(&self) -> &T {
        &self.x
    }
}
