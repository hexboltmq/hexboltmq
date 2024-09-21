use uuid::Uuid;
use tokio::time::{Duration, Instant};
use crate::queue::{Message, Queue};
use tokio::sync::Mutex;
use std::sync::Arc;

/// Represents a producer responsible for sending messages to the queue or cluster.
#[derive(Debug, Clone)]
pub struct Producer {
    id: Uuid,                    // Unique ID for the producer
    queue: Arc<Mutex<Queue>>,     // Reference to the queue
}

impl Producer {
    /// Creates a new producer.
    ///
    /// # Arguments
    ///
    /// * `queue` - The reference to the queue the producer will push messages into.
    ///
    pub fn new(queue: Arc<Mutex<Queue>>) -> Producer {
        Producer {
            id: Uuid::new_v4(),
            queue,
        }
    }

    /// Sends a message to the queue.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the message to be sent.
    /// * `priority` - The priority of the message (higher priority messages will be processed first).
    /// * `delay` - Optional delay for delayed message delivery.
    pub async fn send_message(&self, content: String, priority: u8, delay: Duration) {
        // Generate a message ID by converting Uuid to u64 (use part of Uuid or implement custom logic)
        let message = Message {
            id: Uuid::new_v4().as_u128() as u64, // Convert Uuid to u64
            content,
            priority,
            available_at: Instant::now() + delay, // Calculate availability time based on the delay
            retry_count: 0,    // Default retry count for this message
            max_retries: 5,    // Max retries allowed for this message
        };

        println!("Producer {:?} sending message: {:?}", self.id, message);

        // Push the message to the queue
        let queue = self.queue.clone();
        let mut locked_queue = queue.lock().await;
        locked_queue.push(message, delay).await.unwrap();
    }
}
