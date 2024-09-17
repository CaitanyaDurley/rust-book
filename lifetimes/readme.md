# Lifetimes
Lifetimes are another kind of generic - they are the scope for which a reference is valid.

Instead of ensuring types have methods (like traits), they ensure that references are valid for as long as we need.

Every reference in Rust has a lifetime, usually they are implicit and inferred (like types). We must only annotate lifetimes when the lifetimes of references could be related in multiple ways.

## Dangling references
This is what lifetimes prevent - a program referencing data other than that it is meant to.

## Lifetime annotations
Lifetime annotations don't change the lifetime of a reference. They describe the relationships of the lifetimes of multiple references to each other.
```rust
&i32 // a reference
&'a i32 // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```