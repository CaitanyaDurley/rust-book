fn main() {
    let list_nums = vec![1,2,3];
    // can pass into map a closure
    let list_strings: Vec<String> =
        list_nums.iter().map(|i| i.to_string()).collect();
    // or a function
    let list_strings: Vec<String> =
        list_nums.iter().map(ToString::to_string).collect();
    // including an enum variant initialiser
    let list_status: Vec<Status> =
        list_nums.into_iter().map(Status::Value).collect();

}

// Recall that the name of each enum variant we define automatically
// becomes an initialiser function
enum Status {
    Value(i32),
    Stop,
}

// func accepting a function pointer
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}