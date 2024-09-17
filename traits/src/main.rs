// you must bring a trait into scope in order to use
// its functionality
use traits::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "some tweet".to_string(),
        reply: false,
        retweet: false,
    };
    println!("tweet: {}", tweet.summarise());
    println!("tweet: {}", tweet.summarise_pretty());
}
