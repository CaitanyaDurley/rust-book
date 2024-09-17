// crate references the crate root (i.e. main.rs)
// to reference an external crate use its name instead
// this is an absolute path, can drop the crate:: to make
// it a relative path (relative to current module)
// can use super to begin in the parent module
use crate::garden::vegetables::Asparagus;

// tells the compiler to include the code in src/garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("{:?}", plant);
}
