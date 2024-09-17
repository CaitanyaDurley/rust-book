## Inheritance
In OO programming languages, inheritance is when a subclass inherits all the fields and methods of its parent.

It has recently fallen out of favour because of its tendency to share more code than necessary. Subclasses can inherit methods that don't make sense for it.

However, it is very useful for polymorphism.

### Polymorphism
This is the concept of code that can work with data of multiple types.

In the context of inheritance, this usually means methods defined to accept a base class, but which can then be called with any subclass.

Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what the types must provide.

## Trait objects
However, this isn't always enough. Consider the case where you want a vector of different types, you can't list all the possible types (else you could use an enum) because the user might create their own types, but you only want to allow types implementing some trait. We can do this by defining the vector to take a trait object.

A trait object points to two things:
1. An instance of a type implementing the trait
1. a table used to look up trait methods on that type at runtime

We create a trait object by specifying some sort of pointer (e.g. a & reference or a Box smart pointer) then the `dyn` keyword and finally the relevant trait.

### Dynamic dispatch
When we use trait bounds on generic, the compiler generates non-generic implementations for each concrete type the generic type could be. This is monomorphization and the resulting code is doing _static dispatch_, i.e. the compiler knows which method you're calling at compile time.

When we use trait objects, Rust must use _dynamic dispatch_, i.e. the compiler generates code which will figure out which method to call at runtime. This happens because the compiler doesn't know all the types that might be used with the code using trait objects. At runtime, Rust uses the pointers inside the trait object to know which method to call during runtime. This lookup has a performance cost, as does the loss of optimisations since the compiler can't inline a method's code.