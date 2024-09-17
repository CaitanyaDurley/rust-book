// To directly compile and run while passing command
// line args, do cargo run -- pattern poem.txt
use std::{env, process};
use minigrep::Config;


fn main() {
    // Args is an iterator
    let args = env::args();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
