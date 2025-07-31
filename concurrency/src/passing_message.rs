use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// This program demonstrates passing messages between threads using channels in Rust.
// It creates two threads that send messages to the main thread, which receives and prints them.

pub fn passing_message() {
    // The `mpsc::channel` function creates a channel for sending and receiving messages.
    // The `mpsc` stands for "multiple producer, single consumer".
    let (tx, rx) = mpsc::channel();

    // The `clone` method is used to create multiple senders that can be used in different threads.
    // This allows both threads to send messages to the same receiver.
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
