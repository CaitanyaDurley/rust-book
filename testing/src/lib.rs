// Can only unit test code in lib.rs, not main.rs

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// do cargo test to run the tests in parallel (threads)
// to force consecutive running of tests use --test-threads=1
// to run specific tests use cargo test common_prefix_of_tests
#[cfg(test)]
mod tests {
    use super::*;

    // need #[test] attr so the runner knows it's a test
    // rather than e.g. a common utility used in the tests module
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // a test fails when the function panics (each test is
    // run in a new thread)
    // if a function SHOULD panic then:
    #[test]
    #[should_panic]
    fn doesnt_work() {
        panic!("oh dear");
    }

    // instead of failing on panic, you can fail on
    // an Err Result
    #[test]
    fn another() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("uh oh".to_string())
        }
    }

    // to ignore this test unless we specifically ask
    #[test]
    #[ignore]
    fn expensive_test() {
        ()
    }
}
