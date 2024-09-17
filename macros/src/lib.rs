// The macro_export annotation says the macro should be made available
// whenever this crate is brought into scope. Without this the macro
// couldn't be brought into scope.
#[macro_export]
// the actual defn of vec! in the std lib includes code
// to preallocate the correct amount of memory upfront
macro_rules! vec {
    // ( whole pattern ) => first arm
    // $ declares a variable in the macro system that will
    // contain the Rust code matching the pattern
    // $x:expr matches any expression and calls it $x
    // the comma indicates a literal ',' may optionally appear
    // and the * matches 0+ of the preceeding pattern
    ( $( $x:expr ),* ) => {
        // this is the code emitted by an input matching
        // this pattern. We only have one arm here.
        {
            let mut temp_vec = Vec::new();
            $(
                // the code within $()* is generated
                // for each part that matches $() in
                // the pattern 0+ times. For each match,
                // $x is bound to the relevant value
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
    // any other pattern results in an error
}