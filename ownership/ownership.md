# Ownership
A set of rules that govern how rust manages memory
Some langs have GC that regularly frees no longer used memory, in others the programmer must malloc and free.

In rust memory is managed through a system of ownership with a set of rules the compiler checks. If any rules are violated the program will compile. None of the features of ownership will slow down your program.

# Stack & heap
Any data stored on the stack must have a known fixed size. Data with an unknown size at compile time (or a size which may change) must be stored on the heap.
To put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer (the address of that location).
This is called allocating (pushing values onto the stack is not allocating).
The pointer is a known, fixed size so may be stored on the stack.

Pushing to the stack is faster because the allocator doesn't have to search for a place to store data, and doesn't have to mark mem as in use.
Accessing data in the heap is also slower because you have to follow a pointer, and contemporary processors work faster if the data is close together (which may not be the case on the heap).

When your code calls a function, the parameters passed in (potentially including pointers to data on the heap) and the func's local variables get pushed onto the stack. When the func ends, the values are popped off.

Ownership handles:
- Keeping track of what parts of code are using what data on the heap
- Minimising the amount of duplicate data on the heap
- Cleaning up unused data on the heap

# Ownership rules
- Each value in rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped
