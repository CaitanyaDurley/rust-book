use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

fn main() {
    spawning_thread();
    closures();
    channels();
    mutexes();
}

fn spawning_thread() {
    let h = thread::spawn(|| {
        for i in 1..5 {
            println!("number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..3 {
        println!("number {i} from main thread");
        // while the main thread is sleeping the OS switches threads
        thread::sleep(Duration::from_millis(1));
    }
    // block the main thread until the other thread terminates
    h.join().unwrap();
}

fn closures() {
    let v = vec![1,2,3];
    // we have to move v into the closure (rather than just borrow)
    // since we don't know how long the thread will run so we can't
    // guarantee the reference to v will always be valid
    let h = thread::spawn(move || {
        println!("v = {:?}", v);
    });
    h.join().unwrap();
}

fn channels() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi");
        // send takes ownership of the value because the other thread
        // could modify or drop it
        tx.send(val).unwrap();
    });
    // recv blocks until a value is received. If the transmitter closes,
    // recv will return an Error
    let received = rx.recv().unwrap();
    println!("Got: {received}");
    // try_recv doesn't block - Ok if there's a message waiting, else Err
    println!("Any messages: {}", rx.try_recv().is_ok());
}

fn mutexes() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            // lock blocks until the lock is acquired and panics if another
            // thread holding the lock panicked
            let mut num = counter.lock().unwrap();
            // lock returns a smart pointer MutexGuard
            *num += 1;
            // MutexGuard implements Drop s.t. the lock is released
        });
        handles.push(h);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", counter.lock().unwrap());
}
