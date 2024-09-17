## Newtype pattern
The newtype pattern is also useful for type safety and abstraction - i.e. statically enforcing that values are never confused and indicating the units of a value. E.g. `struct Meters(u32)` specifies the units of the u32, and you couldn't call a function which takes Meters with another unit (e.g. Millimeters) or even just a plain u32.

You can also use the newtype pattern to abstract away implementation details of a type: the new type can expose a different public API to that of the private inner type.

## Type synonyms/aliases
You can use the type keyword to give an existing type another name. E.g. `type Kilometers = i32`. Unlike the newtype example above, Kilometers here is not a separate type, just another name for an i32.

These are commonly used as an abbereviation for a long type name which frequently appears in your codebase. E.g. `type Thunk = Box<dyn Fn() + Send + 'static>`.

## The Never type
Rust has a special type named `!` that's known in type theory as the _empty type_, or in Rust as the _never type_ and stands in the place of the return type when a function will never return.

For example the `panic!` macro has a return type of `!`, as does the `continue` key word (which moves control back to the top of the loop).

## Dynamically sized types
Rust needs to know how much space to allocate for a value of a particular type, and all values of a type must use the same amount of memory. Yet Rust also has dynamically sized types (aka unsized types) whose size can only be known at runtime.

An example of a DST is `str`, and as in this case, in general it is not allowed to create a variable holding a DST. Instead we must put the value behind a pointer, e.g. `&str`, `Box<str>` or `Rc<str>`. Another example is trait objects; to use them in a type they must be behind some kind of pointer, e.g. `Box<dyn Trait>`.

Rust automatically implements the `Sized` trait for every type whose size is known at compile time. Every generic function implicitly has a trait bound to insist the types are Sized. You can relax this trait bound by using the syntax
```
fn relaxed_func<T: ?Sized>(t: &T) {}
```
but note the type of any unsized params must be behind some kind of pointer.