use crossbeam::channel::bounded;
use indexmap::IndexMap;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    // Create a bounded queue with a capacity of 10
    let (sender, receiver) = bounded::<i32>(10);

    // Create an IndexMap wrapped in an RwLock for thread-safe, ordered storage
    let index_map = Arc::new(RwLock::new(IndexMap::new()));

    // Open or create a log file for the IndexMap in JSON format
    let mut index_map_log: PathBuf = env::current_dir().expect("Failed to get current directory");
    index_map_log.push("index_map_log.json");

    // Clone the Arc for the producer thread
    let producer_index_map = Arc::clone(&index_map);

    // Spawn a producer thread
    let producer = thread::spawn(move || {
        for i in 1..=20 {
            match sender.send(i) {
                Ok(_) => {
                    // Add the item to the IndexMap in a write lock
                    {
                        let mut map = producer_index_map.write().unwrap();
                        map.insert(i, i);
                    }
                    // Log the contents of the IndexMap to a JSON file
                    log_index_map_to_json(&producer_index_map, &index_map_log).expect("Failed to log IndexMap to JSON");
                }
                Err(err) => println!("Failed to send: {}", err),
            }
            thread::sleep(Duration::from_millis(2000)); // Simulate work
        }
    });

    // Clone the Arc for the consumer thread
    let consumer_index_map = Arc::clone(&index_map);

    // Spawn a consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(item) = receiver.recv() {
            // Remove the item from the IndexMap in a write lock
            {
                let mut map = consumer_index_map.write().unwrap();
                // map.remove(&item);
            }

            thread::sleep(Duration::from_millis(2000)); // Simulate processing time
        }
    });

    // Wait for both threads to complete
    producer.join().unwrap();
    consumer.join().unwrap();
}

// Function to log the contents of the IndexMap to a JSON file
fn log_index_map_to_json(index_map: &Arc<RwLock<IndexMap<i32, i32>>>, log_file_path: &PathBuf) -> std::io::Result<()> {
    let map = index_map.read().unwrap();

    // Serialize the IndexMap to JSON
    let json_data = serde_json::to_string_pretty(&*map)?;

    // Open the log file and append the JSON data
    let mut file = OpenOptions::new()
        .create(true)  // Create the file if it doesn't exist
        .append(true)  // Append to the file rather than overwrite
        .open(log_file_path)?;

    writeln!(file, "{}", json_data)?;
    writeln!(file, "\n\n")?;  // Add two empty lines between logs
    Ok(())
}
