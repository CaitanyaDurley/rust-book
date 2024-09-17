# Macros
A _macro_ refers to a family of features in Rust: _declarative_ macros with `macro_rules!` and three kinds of _procedural_ macros:
1. Custom `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums - e.g. `#[derive(Debug)]`
1. Attribute-like macros that define custom attributes usable on any item
1. Function-like macros that look like function calls but operate on the tokens specified as their argument - e.g. `dbg!(x)`

## Macros vs functions
Fundamentally, macros are a way of writing code that writes other code, aka _metaprogramming_. Macros expand to produce more code than the code you've written manually. In a sense then, macros are like functions which are called at compile-time rather than runtime.

A function signature must declare the number and type of parameters the function takes. Macros, however, can take a variable number of parameters (e.g. `println!`). Macros are expanded before the compiler interprets the meaning of the code, which is why macros can implement a trait on a type, whereas functions cannot.

Another difference is that macros must be defined/brought into scope _before_ you call them in a file, as opposed to functions which can be defined anywhere and called anywhere.

### Declarative macros
The most widely used form of macros is the declarative macro, aka "macros by example", "macro_rules! macros", or just "macros".

They are similar to a `match` expression in that they compare a value to some patterns and the matching pattern maps to some code. With macros, the value is the literal Rust source code passed into the macro, and the matching code is not run, but instead replaces the code passed to the macro during compilation.

To define a macro, you use the `macro_rules!` construct. In lib.rs there is an example showing a stripped down definition of the `vec!` macro. Note that we can't use a function because `vec!` accepts an indeterminate number of params of an indeterminate type.

### Procedural macros
Procedural macros accept some code as an input, operate on that code and produce some code as an output rather than matching against code and replacing that code like declarative macros.

Currently, procedural macro definitions must reside in their own crate with a special crate type. This is hoped to change in the future.

Go to the procedural_macros crate from here.