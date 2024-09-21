use tokio::sync::Mutex;
use std::sync::Arc;
use crate::queue::Queue;
use uuid::Uuid;
use std::time::Duration;
use tokio::time::sleep;

/// Represents a consumer responsible for retrieving messages from the queue and processing them.
#[derive(Debug, Clone)]
pub struct Consumer {
    id: Uuid,                    // Unique ID for the consumer
    queue: Arc<Mutex<Queue>>,     // Reference to the queue
}

impl Consumer {
    /// Creates a new consumer.
    ///
    /// # Arguments
    ///
    /// * `queue` - The reference to the queue the consumer will pull messages from.
    ///
    pub fn new(queue: Arc<Mutex<Queue>>) -> Consumer {
        Consumer {
            id: Uuid::new_v4(),
            queue,
        }
    }

    /// Consumes a message from the queue and processes it.
    ///
    /// # Arguments
    ///
    /// * `process_message` - A closure that processes the message.
    ///
    /// # Examples
    ///
    pub async fn consume<F>(&self, process_message: F)
    where
        F: Fn(&str) + Send + 'static,
    {
        loop {
            let queue = self.queue.clone();
            let mut locked_queue = queue.lock().await;

            // Attempt to retrieve a message from the queue
            if let Some(message) = locked_queue.pop().await.unwrap() {
                println!("Consumer {:?} processing message: {:?}", self.id, message);
                
                // Process the message using the provided closure
                process_message(&message.content);

                // Acknowledge the message (if the queue supports acknowledgment)
                locked_queue.ack(message.id).await.unwrap();
            } else {
                // If no message is available, wait before retrying
                println!("No messages available, retrying...");
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
