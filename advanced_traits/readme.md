## Placeholder types
Associated types connect a type placeholder with a trait s.t. the trait's method definitions can use these placeholder types in their signatures.

The difference to generics arises in that you can only have one implementation of the trait for your type, whereas a trait defined with generics could be implemented many times (once for each concrete type) on your type.

## Fully qualified syntax
In Rust it is possible to:
1. Name a trait's method the same as another trait's method
1. Implement both such traits on a type
1. Implement a method directly on a type with the same name as methods from traits

To handle disambiguation you need to tell Rust which method you want to call.
By default, Rust will choose the method directly implemented on the type.
To deviate from this, use the trait's name, i.e. `Trait::method(&my_type)`.
But note this only works if the method takes self, if it doesn't then Rust can't know which type you want this method's implementation of. In this case use `<MyType as Trait>::method(other_params)`

## Supertraits
We can write a trait definition that depends on another trait: i.e. for a type to implement your trait it must also implement the other trait. The trait your trait relies on is called a supertrait of your trait.

E.g., you might want to make a PrettyPrint trait that relies on the Display trait's functionality. To do this, define your trait like `trait PrettyPrint: std::fmt::Display {...}`

## Newtype pattern
Recall we're only allowed to implement a trait on a type if at least one of the trait or the type are local to our crate.
It's possible to get around this restriction using the _newtype pattern_, which involves creating a new type in a tuple struct.
The tuple struct has one field and so is just a thin wrapper around the type we want to implement a trait for. The wrapper type is thus local to our crate and we can implement the trait on the wrapper.
There is no performance penalty for using the newtype pattern since the wrapper type is elided at compile time.

One downside of this pattern is that the wrapper is a new type and so doesn't have the methods of the value it holds. If you want just some of the methods of the inner type, so can implement them manually (referencing `wrapper.0`). If you want all of them, you can implement the Deref trait to return the inner type.