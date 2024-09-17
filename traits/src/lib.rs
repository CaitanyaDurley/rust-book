use std::fmt::Display;

// The compiler will enforce that any type implementing
// the Summary trait has a summarise method with this
// exact signature
pub trait Summary {
    fn summarise(&self) -> String;
    // can provide default impl which types can keep
    // or override
    fn summarise_pretty(&self) -> String {
        // Can depend on non-default impl
        format!("Summary: {}", self.summarise())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarise(&self) -> String {
        format!("{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarise(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// item is any type implementing the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarise());
}

// the prevs is syntax sugar for the longer "trait bound"
// syntax
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarise());
}

// T must implement both Summary and Display traits
pub fn notify_v3<T: Summary + Display>(item: &T) {}

// alternative syntax for multiple trait bounds
fn some_func<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Display,
    U: Clone + Display,
{
    1
}

// can also use the "impl Trait" syntax to specify
// a func returns some type that implements the trait
// however, your func must only return a single type
fn returns_summarisable() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: String::from("asdf"),
        reply: false,
        retweet: false,
    }
    // you couldn't conditionally return a NewsArticle
}

// we can use trait bounds to conditionally implement
// methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // cmp_display is only implemented if Pair's type
    // T has the required traits
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest is x = {}", self.x);
        } else {
            println!("Largest is y = {}", self.y);
        }
    }
}

// can conditionally implement a trait for any type
// that implements another trait. E.g. in standard lib
// impl<T: Display> ToString for T {
//     // blah
// }
