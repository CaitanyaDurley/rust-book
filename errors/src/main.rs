// an unrecoverable error causes rust to panic
// if a func can fail due to an err, can return
// a Result<T, E> type
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::error::Error;

// in addition to () main can also return a Result<(), E>
// Box<dyn Error> essentially means any kind of error
// Ok(()) means the exe returns 0, else returns >0
fn main() -> Result<(), Box<dyn Error>> {
    // panic!("uh oh");
    let v = vec![1, 2, 3];
    // v[3]; rust panics to protect against buffer overreads
    let file_result = File::open("dne.txt");
    let file = match file_result {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound =>
                File::create("dne.txt").unwrap_or_else(|error| {
                    panic!("Couldn't create the file: {:?}", error);
            }),
            other_err => panic!("Couldn't open the file: {:?}", other_err),
        }
    };
    // if you just want to panic on err, use Result.unwrap()
    // if you want to just customise the err msg, use Result.expect(msg)
    let file = File::open("hello.txt")?;
    Ok(())
}

// this func propogates errors to its caller, who is better informed on
// how to deal with the err. This pattern is so common that rust provides
// the shortcust operator ?
fn read_name_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");
    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut name = String::new();
    match file.read_to_string(&mut name) {
        Ok(_) => Ok(name),
        Err(e) => Err(e),
    }
}

// the ? operator returns the error from the func if it gets one.
// However it also calls io::Error.from(err) to convert errors of
// any type to the that of the func's return type (assuming the return
// type implements the From<io::Error> trait)
fn read_name_from_file_v2() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut name = String::new();
    file.read_to_string(&mut name)?;
    Ok(name)
}

// the standard library has an easy func for this
fn read_name_from_file_v3() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

// ? can also be used for return type of Option
// if None return None, else consume Some(T)
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
