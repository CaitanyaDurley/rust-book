use std::fmt;

fn main() {
    println!("Hello, world!");
}

// this is the definition of the Add trait in std::ops
// implementing this trait allows you to overload its behaviour
// This trait has a generic Rhs which defaults to Self if you don't
// pass in a concrete type. Note that the generic here is essentially letting
// you define the ability to add different things (the RHS) to your type
trait Add<Rhs = Self> {
    // the associated type Output indicates that for one combo (Self, Rhs)
    // the output is of a fixed type
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Using default for Rhs - i.e. adding two Point instances
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// whereas here we specify the concrete type
impl Add<i32> for Point {
    type Output = Point;

    fn add(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

// Example of a super trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // to_string exists from the Display trait
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// have to impelemnt Display first
impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// after which we can implement OutlinePrint
impl OutlinePrint for Point {}

// We wish to implement Display on Vec<T>
struct Wrapper (
    Vec<String>
);

// so we implement it on the local wrapper
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}