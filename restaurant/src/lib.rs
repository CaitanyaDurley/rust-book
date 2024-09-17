// All items in Rust are private to parent modules
// by default, though children can see items in their
// parent modules
// the pub keyword makes something public to its immediate
// parent only

// hosting and serving are sibling modules
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// this function is in the crate root module
// it needs to be pub for it to be accessible
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    // enum's children are available
    back_of_house::Appetiser::Soup;

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind on the bread
    meal.toast = String::from("Wheat");
    // next line won't compile
    // meal.seasonal_fruit = String::from("apple");
}

mod back_of_house {
    // pub can make a struct/enum public
    pub enum Appetiser {
        Soup,
        Salad,
    }

    // a struct's fields will be private by default
    pub struct Breakfast {
        pub toast: String,
        // say customer can choose the bread but
        // chef chooses fruit based on what's in season
        seasonal_fruit: String,
    }

    // because Breakfast has a private field, we MUST
    // provide a public func in order to be able to
    // create an instance of it (we couldn't set the value
    // of seasonal_fruit otherwise)
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

// brings hosting module into scope, but only for
// the scope in which the use occurs
use crate::front_of_house::hosting;

pub fn eat_at_restaurant_2() {
    // it is idiomatic to bring a func's parent module
    // into scope rather than the func itself, to make it
    // clear that the func isn't locally defined
    hosting::add_to_waitlist();
}
// it is convention to specify the whole path for
// structs, enums and other items
use back_of_house::Breakfast;

// if you want to bring it two types of the same name
// into scope, can either only bring in parent module
// or use as
use std::io::Result as IoResult;
// use only brings into this module's scope, if you want
// it public, use re-exporting
pub use std::fmt::Result;
// can use nested paths to reduce no of lines
use std::{cmp::Ordering, io};
// to bring in all public items use the glob operator
use std::collections::*;
// glob is often used for tests
