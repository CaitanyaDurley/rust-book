// unlike Vec and String, the HashMap is not auto brought
// into scope in the prelude
use std::collections::HashMap;

pub fn main() {
    let mut scores = HashMap::new();
    // types that implement the Copy trait are copied
    // into the hashmap: like the i32 in the value
    scores.insert("Blue".to_string(), 10);
    // owned values are moved into the hashmap, like
    // the String in the key
    scores.insert("Yellow".to_string(), 50);
    let team_name = String::from("Blue");
    let score = scores
        .get(&team_name)
        .copied()
        .unwrap_or(0);
    for (key, value) in &scores {
        // while we can iter, the order of the
        // kv pairs is random
        println!("{key}: {value}");
    }
    inserting_references();
    // overwrites the prevs value of Yellow
    scores.insert("Yellow".to_string(), 20);
    // only inserts if the key doesn't exist, always
    // returns a mut ref to the value
    let yellow = scores
        .entry("Yellow".to_string())
        .or_insert(50);
    println!("{yellow}");
    update_value();
}

fn update_value() {
    let text = "hello world and hello";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map
            .entry(word)
            .or_insert(0);
        *count += 1;
    }
    dbg!(map);
}

fn inserting_references() {
    let my_str = "abc".to_string();
    let ptr = &my_str;
    let mut d = HashMap::new();
    // inserting a reference obvs does not move the
    // actual value, instead the reference (which
    // implements Copy) is copied in
    d.insert(ptr, 1);
    // ptr is still valid
    dbg!(ptr);
    // the value the references point to must be valid
    // for at least as long as the hash map
}