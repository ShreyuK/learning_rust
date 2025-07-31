mod creating_threads;
use creating_threads::creating_threads;

mod passing_message;
use passing_message::passing_message;

fn main() {
    // Call the function to demonstrate thread creation and joining
    println!("Starting thread creation example:");
    println!("----------------------------------");
    creating_threads();

    println!("\nStarting message passing example:");
    println!("----------------------------------");
    // Call the function to demonstrate passing messages between threads
    passing_message();
}
