# Procedural macros
This follows from the macros section/crate.

As a declarative macro is similar to a match statement, a procedural macro is similar to a function. The general definition of a procedural macro looks like
```rust
use proc_macro::TokenStream;
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    // operate on the TokenStream here and return another
}
```
some_attribute is a placeholder for using a specific macro variety.
The attribute attached to the function specifies which kind of procedural macro we're creating.

## Custom `derive` macro
Let's create a `HelloMacro` trait with an associated function. Rather than making our users implement the trait for each of their types, we'll provide a procedural macro so they can just annotate their type with `#[derive(HelloMacro)]` to get a default implementation of the function.

While a default implementation is possible without the macro, in our case we want the default implementation to use the name of the trait.

As procedural macros must be defined in their own crate, convention is to create a new lib crate called yourCrateName_derive within your crate's directory. Note the three new crates in _procedural\_macros\_derive/Cargo.toml_:
1. `proc_macro` - comes with Rust, is the compiler's API that allows us to read & manipulate Rust code from within our code
1. `syn` - parses Rust code from a string into a syntax tree
1. `quote` - turns `syn` data structures back into Rust code

In particular `syn::parse` returns a DeriveInput struct which looks like:
```rust
DeriveInput {
    // --snip--
    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

## Attribute-like macros
These are like custom derive macros, but instead of generating code for the `derive` attribute, they allow you to create new attributes of your own.
Attribute-like macros can also be applied to items other than structs and enums, e.g. functions.

For example, you might have an attribute named route that annotates functions in a web app framework
```rust
#[route(GET, "/")]
fn index() {...}
```

The `#[route]` attribute would be defined by the framework as a procedural macro. The signature of the macro definition would look like
```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {...}
```
where `attr` holds the contents of the attribute (i.e. `GET, "/"`) and `item` holds the definition of `index`.

## Function-like macros
Function-like macros are very similar to `macro_rules!` macros, in that they look like function calls. However, `macro_rules!` macros can be defined only using hte match-like syntax, whereas function-like macros, being procedural macros, take a `TokenStream` parameter and manipulate it.

An example could be a `sql!` macro that is called like
```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```
and parses the SQL inside to check it is syntactically correct at compile time. This processing is too complex for a `macro_rules!` macro.
The `sql!` macro would be defined as
```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {...}
```
