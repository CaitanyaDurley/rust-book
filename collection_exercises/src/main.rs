use std::collections::HashMap;
use std::io;
use std::str;

const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

fn main() {
    let v = vec![2, 1, 3, 1, 4];
    let med_and_mod = median_and_mode(&v);
    dbg!(med_and_mod);
    println!("{}", pig_latin("first"));
    println!("{}", pig_latin("apple"));
    employee_names();
}

fn median_and_mode(v: &Vec<i32>) -> (i32, i32) {
    let mut v = v.clone();
    v.sort_unstable();
    let med_ix = (1 + v.len()) / 2;
    let mut counts = HashMap::new();
    for value in &v {
        let c = counts.entry(value).or_insert(0);
        *c += 1;
    }
    let mut mode = &v[0];
    for (key, value) in counts.iter() {
        if *value > counts.get(mode).copied().unwrap() {
            mode = key;
        }
    }
    (v[med_ix], *mode)
}

fn pig_latin(s: &str) -> String {
    let mut out = String::new();
    for word in s.split_whitespace() {
        let mut chars = word.chars();
        let first = chars.next().unwrap();
        let first_is_consonant = CONSONANTS.contains(first);
        if !first_is_consonant {
            out.push(first)
        }
        for c in chars {
            out.push(c);
        }
        out.push('-');
        if first_is_consonant {
            out.push(first);
            out.push_str("ay");
        } else {
            out.push_str("hay");
        }
        out.push(' ');
    }
    out.trim_end().to_string()
}

fn employee_names() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        let input = input.trim();
        let mut chars = input.split_whitespace();
        let cmd = chars.next().unwrap();
        match cmd {
            "Add" => add_employee(&mut departments, chars),
            "Get" => println!("{:#?}", get_department(&mut departments, chars)),
            &_ => println!("Bad command"),
        }
    }
}

fn get_department(departments: &mut HashMap<String, Vec<String>>, mut chars: str::SplitWhitespace<'_>) -> Vec<String> {
    let dep = chars.next().unwrap().to_string();
    let names = departments.get(&dep);
    let mut names = match names {
        Some(v) => v.clone(),
        None => vec![],
    };
    names.sort_unstable();
    names
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>, mut chars: str::SplitWhitespace<'_>) {
    let name = chars.next().unwrap().to_string();
    assert_eq!(chars.next().unwrap(), "to");
    let dep = chars.next().unwrap().to_string();
    departments.entry(dep).or_insert(vec![]).push(name);
}

