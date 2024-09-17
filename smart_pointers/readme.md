1. A pointer is a general concept for a variable that contains an address in memory.
1. Rust references are a kind of pointer.
1. References borrow the value they point to, but have no other special capabilities and also have no overhead
1. Smart pointers are data structures that act like a pointer but also have additional metadata/capabilities.
1. References only borrow data, while smart pointers often own the data they point to
1. String and `Vec<T>` are both smart pointers. E.g. String stores its capacity (as metadata) and has the capability to ensure its data is always valid UTF-8
1. Smart pointers are usually implemented via structs which implement the Deref (so we can access the data) and Drop (so we can drop the data) traits.

## Box<T>
THe Box<T> type is a smart pointer which allows you to store data on the heap (rather than the stack). The pointer to the heap data remains on the stack.
Boxes have no performance overhead (other than their data being on the heap) but also don't have many capabilities. They are most ofen used:
1. When you have a type whose size can't be known at compile time, but you need to use a value of that type in a context requiring an exact size
1. When you want to transfer ownership of a large amount of data (on the stack) without copying the data
1. When you want to own a value but you only care that it implements a particular trait, rather than the specific type it may be

### Recursive types
A recursive type's value can contain another value of the same type as part of itself. These are problematic because their space is not known at compile time. But because boxes have a known size, we can enable recurisve types by inserting a box in the definition.
#### Cons list
This is a recursive type commonly found in functional languages. It is Lisp's version of a linked list and is constructed of nested pairs, e.g.
```
(1, (2, (3, Nil)))
```

## Deref
Implementing the Deref trait allows you to customise the behaviour of the dereference operator *. The trick is to implement in such a way that a smart pointer can be treated like a regular reference, so that code which operates on references can also work with smart pointers.

### Deref coercion
Deref coercion converts a reference to a smart pointer into a reference to the underlying datatype. E.g. &String to &str (because String implements deref such that it returns &str).
This feature of Rust lets us write functions of references to data, and call that same function with a (reference to) a smart pointer.

#### DerefMut
The DerefMut trait overrides the * operator on mutable references.
Rust does deref coercion:
1. From `&T` to `&U` when `T: Deref<Target=U>`
1. From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
1. From `&mut T` to `&U` when `T: Deref<Target=U>`
The third case reflects that it is safe to coerce a mutable reference to an immutable reference, but not vice versa.

## Drop
The Drop trait lets you customise what happens when a value is about to go out of scope. E.g. when a `Box<T>` is dropped, it will deallocate the space on the heap that the Box points to.

## `Rc<T>`
Usually a value has just one owner. However, there are cases when a single value might have multiple owners. E.g. in a graph, a node is conceptually owned by all the edges that point to it - a node shouldn't be cleaned up until it has no edges pointing to it.

To enable multiple ownership in Rust, use the `Rc<T>` type - this keeps track of the number of references to a value to determine whether or not the value is still in use.

Use the Rc type when you want to allocate data on the heap for multiple parts of the program to read and we can't know which part will finish using the data last at compile time (if we knew which would finish last, just make that part the owner). Rc should only be used in single-threaded scenarios.

Use `Rc::clone` to increment the reference count of an Rc object - note that unlike most types, Rc's implementation of clone doesn't actually make a deep copy of the data, it just clones the pointer itself (and so is fast).

## Interior mutability
This is a design pattern that allows you to mutate data even when there are immutable references to that data. Normally this is disallowed by the borrowing rules, to bend these rules we use `unsafe` code to tell the compiler that the rules will be followed at runtime, even though the compiler can't guarantee it.
The unsafe code is wrapped in a safe API, and the outer type is still immutable.

The `RefCell<T>` type follows the interior mutability pattern.

### `RefCell<T>`
This type allows you to mutate the value inside the RefCell, even when the RefCell is itself immutable.

Recall the borrowing rules:
1. You can have ONE mutable reference OR you can have any number of immutable references, but you can't have both
1. References must always be valid

Like `Box<T>` and references, `RefCell<T>` represents single ownership over the data it holds, but unlike Box and references, the borrowing rules are enforced at runtime. If you break the rules, your program will panic and exit.
Like `Rc<T>`, RefCell is only for use in single-threaded scenarios.

Once you have a RefCell type, use the `borrow` and `borrow_mut` methods to get smart pointers (`Ref<T>` and `RefMut<T>` resp.) to the underlying data.
The RefCell keeps track of how many such smart pointers are currently active, and can hence enforce the borrowing rules at runtime.

### Multiple owners of mutable data
Recall Rc lets you have multiple owners of some data, but only allows immutable access to that data. If you have an Rc holding a RefCell, you can mutate a value with multiple owners!

## Reference cycles
It is not impossible to create memory that is never cleaned up - memory leaks are "memory safe" according to Rust's memory safety guarantees.

### Non ownership relationships
We can avoid reference cycles by having cycles in which some relationships aren't ownership. Calling `Rc::clone` increases the `strong_count` of an Rc instance. We can also create a weak reference to the value within Rc by calling `Rc::downgrade`. This doesn't increase the strong_count, and so cycles involving weak references will be broken once the strong count is 0.

`Rc::downgrade` returns a smart pointer of type `Weak<T>` and increases the `weak_count` (though the weak_count is not relevant for cleanup purposes). Since the weak pointer might point to a dropped value, you have to call the `upgrade` method on a `Weak<T>` instance to get an `Option<Rc<T>>`.