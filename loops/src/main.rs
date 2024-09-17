fn main() {
    println!("Hello, world!");
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            // break can return an expression
            break count * 2;
        }
    };
    println!("res={result}");
    count = 0;
    // loop label
    'outer_loop: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                // breaks inner loop
                break;
            }
            if count == 2 {
                // references loop label
                break 'outer_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count={count}");
    while count > 0 {
        println!("count={count}");
        count -= 1;
    }
    for elem in (1..4).rev() {
        println!("elem={elem}");
    }
    for n in 1..10 {
        let ans = fib(n);
        println!("ans={ans}");
    }
}

fn fib(n: i8) -> i32 {
    if n <= 0 {
        return -1;
    }
    if (n == 1) | (n == 2) {
        return 1;
    }
    let mut f = [1,1];
    let mut s = 0;
    for _ in 2..n {
        s = f[0] + f[1];
        f[0] = f[1];
        f[1] = s;
    }
    s
}
