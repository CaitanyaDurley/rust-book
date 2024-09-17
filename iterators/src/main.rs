fn main() {
    let v1 = vec![1, 2];
    // the iterator must be mutable to call next
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), None);
    
    // this doesn't need to be mutable because
    // sum takes ownership of the iterator
    let v1_iter = v1.iter();
    assert_eq!(3, v1_iter.sum());
    // can't use v1_iter again

    // methods can take an iterator without consuming
    // it, for example map
    let v1_iter = v1.iter();
    // map does take ownership of v1_iter though
    let _ = v1_iter.map(|x| x + 1)
        .collect::<Vec<i32>>();
}
