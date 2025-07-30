mod deref_trait;
use deref_trait::deref_trait;

mod drop_trait;
use drop_trait::drop_trait;

fn main() {
    // Call the functions to demonstrate deref and drop traits.
    // The deref_trait function shows how to use the Deref trait with a custom smart pointer.
    println!("Calling deref_trait function:");
    deref_trait();

    // The drop_trait function demonstrates the Drop trait with a custom smart pointer.
    println!("Calling drop_trait function:");
    drop_trait();
}
