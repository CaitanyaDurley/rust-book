# Fearless concurrency
## Intro
On most operating systems a program's code is run in a process, with the OS managing multiple processes at once. Within a process, you can have multiple threads runnning simultaneously - the OS will switch execution between them. Threading can improve performance but can also cause problems like:
1. race conditions - threads access data/resources in an inconsistent order
1. deadlocks - two threads are waiting for each other, so neither continue
1. hard to reproduce bugs

The Rust standard library uses a 1:1 model of thread implementation - i.e. the program uses one OS thread per one language thread. There are crates that implement other models of threading.

## Spawning a new thread
To create a new thread call `thread::spawn` and pass the closure you want to run.
Note that when the main thread of a Rust program completes, all spawned threads are terminated, regardless of whether they have finished running.

## Message passing
This is when threads/actors send eeach other messages containing data
> Do not communicate by sharing memory, share memory by communicating
The std library provides an implementation of channels - a means of sending data from one thread to another.

### MPSC
multi-producer, single-consumer - i.e. a channel can have multiple sender but only one receiving end.

## Shared memory
Rather than channels, we can also share memory between threads using multiple ownership.

### Mutexes
Mutual exclusion - only one thread can access some data at a time. To access the data in a mutex, a thread must acquire the mutex's lock. Once it's done with the data that the mutex guards, the thread must unlock the data.

Mutexes in Rust handle releasing the lock for you by wrapping the value obtained after a lock in a smart pointer, `MutexGuard`, which implements the Drop trait s.t. the lock is released. It also provides interior mutability so that you can modify the value starting from an immutable reference to the Mutex.

#### Atomic reference counting
Since captured values in a closure passed to `thread::spawn` must be moved, we need a way for multiple closures to have ownership of the Mutex. `Rc<Mutex<T>>` would be perfect for this, but in fact the Rc type cannot be sent between threads safely because it doesn't implement the `Send` trait.

The `Arc<T>` type is exactly like Rc except it is safe to use in concurrent situations. The reason we don't always use Arc is because thread safety comes with a performance cost you only want to pay when necessary.

## Sync and Send
These traits in the `std::marker` module are what define concurrency in Rust. As marker traits they don't have any methods to implement. Any types made up entirely of one of these traits automatically inherits these traits. Manually implementing these traits on types not made up of Send/Sync parts requires unsafe code and careful thought.

### Send
The Send marker trait indicates that ownership of values of this type can be transferred between threads. This is the case for most Rust types, but not all, e.g. Rc cannot be Send because after cloning the Rc and moving it to another thread, both threads might try to update the reference count at the same time (and this update operation is not atomic for regular Rc).

### Sync
The Sync market trait indicates that it is safe for this type to be referenced from multiple threads. In other words, T is Sync if &T is Send.
Rc is not Sync for the same reasons it is not Send. RefCell is also not Sync because the runtime borrow checking is not thread safe. The Mutex smart pointer is Sync.