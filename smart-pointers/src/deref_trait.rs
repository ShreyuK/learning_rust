use std::ops::Deref;

// This code demonstrates the implementation of a custom smart pointer in Rust.
struct MyBox<T>(T);

// The `MyBox` struct is a simple wrapper around a value of type `T`.
// It can be used to create a smart pointer that behaves like a reference.
// The `MyBox` struct is generic, meaning it can hold any type `T`.
// The `new` function is a constructor for creating instances of `MyBox`.
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// The `Deref` trait allows us to customize the behavior of the dereference operator `*`.
// By implementing `Deref`, we can make `MyBox` behave like a reference to the value it holds.
// This allows us to use `*` to access the value inside `MyBox`.
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn deref_trait() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // Here we use the dereference operator `*` to access the value inside `MyBox`.
    // The `Deref` implementation allows us to treat `y` like a reference to `x`.
    // bts Rust ran *(y.deref())
    assert_eq!(5, *y);
}
