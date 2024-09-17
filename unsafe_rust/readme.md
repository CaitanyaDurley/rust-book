# Unsafe
This is code which doesn't have Rust's memory safety guarantees enforced at compile time.
Unsafe Rust exists because static analysis might sometimes reject a valid program, and also because it's necessary for low-level systems programming (interacting with the OS, or even writing your own OS).

## Unsafe superpowers
Within an unsafe block you can do 5 things that you can't in safe Rust:
1. Dereference a raw pointer
1. Call an unsafe function or method
1. Access or modify a mutable static variable
1. Implement an unsafe trait
1. Access fields of `union`s

Note the borrow checker and Rust's other safety checks still apply within an unsafe block - you only gain access to these five features that are then not checked by the compiler for memory safety.

It's best to enclose unsafe code within a safe abstraction and provide a safe API (as parts of the std library does). This is to prevent uses of unsafe leaking into all the places you/users want to use the functionality - using a safe abstraction is safe.

### Dereferencing a raw pointer
The compiler ensures that references are always valid. Unsafe Rust has raw pointers, which may be immutable or mutable, and are written as `*const T` and `*mut T` resp. The asterisk isn't the dereference operator here but part of the type name. In the context or raw pointers, immutable means that the pointer can't be directly assigned to after being dereferenced.

Unlike references and smart pointers, raw pointers:
1. Are allowed to ignore the borrowing rules (having both immutable and mutable pointers or multiple mutable pointers to the same location)
1. Aren't guaranteed to point to valid memory
1. Are allowed to be null
1. Don't implement any automatic cleanup

### Accessing or Modifying a mutable static variable
In Rust, global variables are called static variables. They are similar to constants, except:
1. static vars can only hold references with the `'static` lifetime
1. Immutable static vars have a fixed address in memory - using the value always accesses exactly the same data. Constants are allowed to duplicate their data whenever used
1. Static vars can be mutable, though accessing and modifying mutable static vars is unsafe

Mutable data that is globally accessible is considered unsafe because it's difficult to ensure there are no data races.

### Implementing an unsafe trait
A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify.
As with unsafe functions, if we implement this trait, we must use the unsafe keyword to tell the compiler we'll uphold the invariants the compiler can't enforce for us.

As an example, you might have a type containing a type that is not Send or Sync (e.g. raw pointers), we can mark the type as Send/Sync using unsafe. Rust can't verify our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads, so we need to do those checks manually and indicate we've done so with `unsafe`.

### Accessing fields of a union
A union is similar to a struct but only one declared field is used in a particular instance at one time.
Unions are primarily used to interface with unions in C code. Accessing union fields is unsafe because Rust can't guarantee the type of the data currently being stored in a union instance.