use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

// The `reference_count` function demonstrates how reference counting works with `Rc`.
// `Rc` is a reference-counted smart pointer that enables multiple ownership of data.
// This allows us to see how many references exist to the data at any point in time
// and helps manage memory safely in a multi-owner scenario.
// This is useful for shared ownership of data, such as in trees or graphs.
pub fn reference_count() {
    // Create a new `Rc` instance that holds a `List`.
    // The `Rc::new` function creates a new reference-counted pointer.
    // The `Cons` variant holds an integer and another `Rc<List>`.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // The `strong_count` method returns the number of strong references to the data.
    println!("Count after creating a = {}", Rc::strong_count(&a));

    // The `Rc::clone` function increments the reference count.
    let _b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        // Create another `Rc` instance that holds a reference to `a`.
        // This demonstrates that we can have multiple owners of the same data.
        let _c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}
