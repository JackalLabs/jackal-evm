// main.rs
use crossbeam::channel::{bounded, unbounded};
use std::thread;
use std::time::Duration;
use tokio::time::{sleep};


fn main() {
    // Create a bounded queue with a capacity of 10
    let (sender, receiver) = bounded::<i32>(10);

    // Spawn a producer thread
    let producer = thread::spawn(move || {
        for i in 1..=20 {
            match sender.send(i) {
                Ok(_) => println!("Produced: {}", i),
                Err(err) => println!("Failed to send: {}", err),
            }
            thread::sleep(Duration::from_millis(2000)); // Simulate work
        }
    });

    // Spawn a consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(item) = receiver.recv() {
            println!("Consumed: {}", item);
            thread::sleep(Duration::from_millis(2000)); // Simulate processing time
        }
    });

    // Wait for both threads to complete
    producer.join().unwrap();
    consumer.join().unwrap();
}
