use crossbeam::channel::bounded;
use dashmap::DashMap;
use serde::Serialize;
use serde_json::json;
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

    // Open or create a log file for the DashMap in JSON format
    let mut dashmap_log: PathBuf = env::current_dir().expect("Failed to get current directory");
    dashmap_log.push("dashmap_log.json");

    // Spawn a producer thread
    let producer = thread::spawn(move || {
        for i in 1..=20 {
            match sender.send(i) {
                Ok(_) => {
                    // Add the item to the DashMap
                    producer_dashmap.insert(i, i);
                    // Log the contents of the DashMap to a JSON file
                    log_dashmap_to_json(&producer_dashmap, &dashmap_log).expect("Failed to log DashMap to JSON");
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
            // // Remove the item from the DashMap
            // consumer_dashmap.remove(&item);

            thread::sleep(Duration::from_millis(2000)); // Simulate processing time
        }
    });

    // Wait for both threads to complete
    producer.join().unwrap();
    consumer.join().unwrap();
}

// Function to log the contents of the DashMap to a JSON file
fn log_dashmap_to_json(dashmap: &Arc<DashMap<i32, i32>>, log_file_path: &PathBuf) -> std::io::Result<()> {
    let mut map = std::collections::HashMap::new();

    // Collect DashMap contents into a HashMap
    for entry in dashmap.iter() {
        map.insert(*entry.key(), *entry.value());
    }

    // Serialize the HashMap to JSON
    let json_data = serde_json::to_string_pretty(&map)?;

    // Open the log file and append the JSON data
    let mut file = OpenOptions::new()
        .create(true)  // Create the file if it doesn't exist
        .append(true)  // Append to the file rather than overwrite
        .open(log_file_path)?;

    writeln!(file, "{}", json_data)?;
    writeln!(file, "\n\n")?;  // Add two empty lines between logs
    Ok(())
}