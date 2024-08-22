use crossbeam::channel::bounded;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Create a bounded queue with a capacity of 10
    let (sender, receiver) = bounded::<i32>(10);

    // Create a Mutex-protected file handle for logging
    let log_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open("production_log.txt")
            .expect("Failed to open log file"),
    ));

    // Spawn a producer thread
    let producer_log_file = Arc::clone(&log_file);
    let producer = thread::spawn(move || {
        for i in 1..=100 {
            match sender.send(i) {
                Ok(_) => {
                    println!("Produced: {}", i);
                    let mut file = producer_log_file.lock().unwrap();
                    writeln!(file, "Produced: {}", i).expect("Failed to write to log file");
                }
                Err(err) => {
                    println!("Failed to send: {}", err);
                    let mut file = producer_log_file.lock().unwrap();
                    writeln!(file, "Failed to send: {}", err).expect("Failed to write to log file");
                }
            }
            thread::sleep(Duration::from_millis(500)); // Simulate work
        }
    });

    // Spawn a consumer thread
    let consumer_log_file = Arc::clone(&log_file);
    let consumer = thread::spawn(move || {
        while let Ok(item) = receiver.recv() {
            println!("Consumed: {}", item);
            let mut file = consumer_log_file.lock().unwrap();
            writeln!(file, "Consumed: {}", item).expect("Failed to write to log file");
            thread::sleep(Duration::from_millis(500)); // Simulate processing time
        }
    });

    // Wait for both threads to complete
    producer.join().unwrap();
    consumer.join().unwrap();
}
