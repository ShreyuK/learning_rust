struct CustomSmartPointer {
    data: String,
}

// Implementing the Drop trait for CustomSmartPointer.
// The `drop` method is called automatically when an instance of CustomSmartPointer
// goes out of scope, allowing us to perform cleanup actions.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// This is useful for managing resources like memory, file handles, etc.
pub fn drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
