use crossbeam::channel::bounded;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a bounded queue with a capacity of 10
    let (sender, receiver) = bounded::<i32>(10);

    // Create a shared vector wrapped in Arc and Mutex for thread-safe access
    let shared_vec = Arc::new(Mutex::new(Vec::new()));

    // Clone the Arc for the producer thread
    let producer_vec = Arc::clone(&shared_vec);

    // Spawn a producer thread
    let producer = thread::spawn(move || {
        for i in 1..=20 {
            match sender.send(i) {
                Ok(_) => {
                    println!("Produced: {}", i);
                    // Add the item to the vector
                    let mut vec = producer_vec.lock().unwrap();
                    vec.push(i);
                }
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

    // Print the contents of the vector after all operations
    let vec = shared_vec.lock().unwrap();
    println!("Final contents of the vector: {:?}", *vec);
}
