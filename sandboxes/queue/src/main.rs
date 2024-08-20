use crossbeam::channel::bounded;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    // Get the current working directory and append the file name
    let mut items_added: PathBuf = env::current_dir().expect("Failed to get current directory");
    items_added.push("items_added.txt");

    // Create a bounded queue with a capacity of 10
    let (sender, receiver) = bounded::<i32>(10);

    // Spawn a producer thread
    let producer = thread::spawn(move || {
        for i in 1..=20 {
            match sender.send(i) {
                Ok(_) => {
                    println!("Produced: {}", i);
                    // Append the produced item to the file
                    save_to_file(&items_added, i).expect("Failed to write to file");
                }
                Err(err) => println!("Failed to send: {}", err),
            }
            thread::sleep(Duration::from_millis(2000)); // Simulate work
        }
    });

    // Get the current working directory and append the file name
    let mut items_removed: PathBuf = env::current_dir().expect("Failed to get current directory");
    items_removed.push("items_removed.txt");

    // Spawn a consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(item) = receiver.recv() {
            println!("Consumed: {}", item);
            save_to_file(&items_removed, item).expect("Failed to write to file");
            thread::sleep(Duration::from_millis(2000)); // Simulate processing time
        }
    });

    // Wait for both threads to complete
    producer.join().unwrap();
    consumer.join().unwrap();
}

// Function to append the item to the file
fn save_to_file(file_path: &PathBuf, item: i32) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)  // Create the file if it doesn't exist
        .append(true)  // Append to the file rather than overwrite
        .open(file_path)?;

    writeln!(file, "{}", item)?;
    Ok(())
}
