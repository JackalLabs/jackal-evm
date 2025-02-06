use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

// Add this struct to manage the queue
pub(crate) struct BoundedQueue {
    queue: VecDeque<String>,
    max_size: usize,
}

impl BoundedQueue {
    pub(crate) fn new(max_size: usize) -> Self {
        BoundedQueue {
            queue: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub(crate) fn enqueue(&mut self, value: String) {
        if self.queue.len() < self.max_size {
            self.queue.push_back(value);
        } else {
            // If the queue is full, handle overflow (e.g., drop the value or replace the oldest)
            println!("Queue is full, dropping value: {}", value);
        }
    }

    pub(crate) fn dequeue(&mut self) -> Option<String> {
        self.queue.pop_front()
    }
}