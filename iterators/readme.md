# Iterators
Iterators in rust are lazy.

All iterators implement a trait named Iterator defined in the std lib. This trait requires implementors to define a next method. The next method returns an Option, which is None when the iteration is over.

Note that calling next on an iterator hence consumes it.

To get an iterator from a collection, call:
1. iter - to get an iterator of immutable references
1. iter_mut - to get an iterator of mutable references
1. into_iter - to get an iterator of owned values (this iterator will take ownership of the collection)
