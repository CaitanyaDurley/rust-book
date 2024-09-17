# Closures
closures are anonymous functions you can save in a variable and pass as arguments to other functions.

Unlike functions, closures can capture values from the scope in which they're defined.

Closures can capture values from their environment in 3 ways:
1. borrowing immutably
1. borrowing mutably
1. taking ownership - this requires the move keyword

The body of a closure can later move a captured value out of the closure (when it is executed), or not. Depending on how the body handles the values, the closure will automatically implement any combination of these three Fn traits:
1. FnOnce applies to closures that can be called at least once - so all closures implement this. If a closure moves captured values out of its body, then it will not implement any of the other Fn traits because it can only be called once.
1. FnMut applies to closures that don't move captured values out of their body, but might mutate the captured values. These closures can be called more than once.
1. Fn applies to closures that don't move captured values out of their body and don't mutate captured values, including if the closures captures nothing from their environment. These closures can be called more than once without mutating their environment - important for concurrency.
