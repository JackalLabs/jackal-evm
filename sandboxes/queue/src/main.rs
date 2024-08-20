use crossbeam::channel::bounded;
use dashmap::DashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::env;

fn main() {

    // Create a bounded queue with a capacity of 10
    let (sender, receiver) = bounded::<i32>(10);

    // Create a shared DashMap to store items in memory
    let dashmap = Arc::new(DashMap::new());

    // Clone the Arc for the producer thread
    let producer_dashmap = Arc::clone(&dashmap);

    // Spawn a producer thread
    let producer = thread::spawn(move || {
        for i in 1..=20 {
            match sender.send(i) {
                Ok(_) => {
                    println!("Produced: {}", i);
                    // Add the item to the DashMap
                    producer_dashmap.insert(i, i);
                }
                Err(err) => println!("Failed to send: {}", err),
            }
            thread::sleep(Duration::from_millis(2000)); // Simulate work
        }
    });

    // Clone the Arc for the consumer thread
    let consumer_dashmap = Arc::clone(&dashmap);

    // Spawn a consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(item) = receiver.recv() {
            println!("Consumed: {}", item);

            // // Remove the item from the DashMap
            // consumer_dashmap.remove(&item);

            thread::sleep(Duration::from_millis(2000)); // Simulate processing time
        }
    });

    // Wait for both threads to complete
    producer.join().unwrap();
    consumer.join().unwrap();
}


