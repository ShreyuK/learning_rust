use std::thread;
use std::time::Duration;

// This program demonstrates spawning a thread in Rust and having both the main thread
// and the spawned thread print messages concurrently. The main thread waits for the
// spawned thread to finish using `join`.
pub fn creating_threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for the spawned thread to finish
    // Uncomment the next line to wait for the spawned thread to finish before proceeding
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
