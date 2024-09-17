## Function pointers
In addition to passing closures to functions, you can also pass regular functions. This is useful when you want to pass a function you've already defined.

Functions coerce to the type `fn` (not to be confused with the `Fn` closure trait), which is a _function pointer_.

Function pointers implement all three of the closure traits (Fn, FnMut and FnOnce) because they don't actually capture and values. This means you can always pass a function pointer to a function that expects a closure. Hence, you should always write functions using a generic type and one of the closure trait bounds so your functions can accept either closures or function (pointers).

The one exception is when interfacing with external code that doesn't have closures, e.g. C. In this case you'd want your function to accept `fn` only.

## Returning closures
Closures don't have a concrete type, they are defined by their implementation of the closure traits. Hence, to return a closure from a function, you should use a trait object as the return type.