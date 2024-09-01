use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::{sleep, Duration, Instant};

/// A message that can be added to the queue.
///
/// Each message has an ID, content, a priority, and a delay.
/// The priority determines the order in which messages are processed, 
/// with higher values being processed first. The delay specifies how long 
/// the message should wait before becoming available for processing.
#[derive(Debug, Clone, Eq)]
pub struct Message {
    /// The unique identifier of the message.
    pub id: u64,
    /// The content of the message.
    pub content: String,
    /// The priority of the message. Higher values indicate higher priority.
    pub priority: u8,
    /// The time when the message will be available for processing.
    pub available_at: Instant,
}

// Implement ordering for the message to be used in a priority queue.
impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare based on availability time first, then priority
        if self.available_at == other.available_at {
            other.priority.cmp(&self.priority) // Higher priority messages come first
        } else {
            self.available_at.cmp(&other.available_at) // Earlier availability comes first
        }
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Custom errors that can occur when interacting with the queue.
#[derive(Debug)]
pub enum QueueError {
    /// Error occurring when a lock cannot be acquired.
    LockError,
}

/// A thread-safe priority queue for managing `Message` objects with support for delayed processing.
///
/// The `Queue` allows multiple producers and consumers to safely push and pop messages concurrently,
/// with messages being processed in order of their priority and availability time.
#[derive(Debug, Clone)]
pub struct Queue {
    messages: Arc<Mutex<BinaryHeap<Message>>>,
}

impl Queue {
    /// Creates a new, empty `Queue`.
    ///
    /// # Examples
    ///
    ///
    /// use hexboltmq::queue::Queue;
    /// let queue = Queue::new();
    ///
    pub fn new() -> Self {
        Queue {
            messages: Arc::new(Mutex::new(BinaryHeap::new())),
        }
    }

    /// Adds a message to the queue with an optional delay.
    ///
    /// Messages are stored based on their priority and availability time.
    /// 
    /// # Arguments
    ///
    /// * `message` - The message to add to the queue.
    /// * `delay` - The delay duration after which the message becomes available for processing.
    ///
    /// # Errors
    ///
    /// Returns `QueueError::LockError` if the queue lock cannot be acquired.
    ///
    /// # Examples
    ///
    ///
    /// use hexboltmq::queue::{Queue, Message};
    /// use tokio::time::Duration;
    /// let queue = Queue::new();
    /// queue.push(Message { id: 1, content: String::from("Hello"), priority: 5 }, Duration::from_secs(2)).await.unwrap();
    ///
    pub async fn push(&self, message: Message, delay: Duration) -> Result<(), QueueError> {
        // Calculate the availability time based on the current time and delay
        let available_at = Instant::now() + delay;

        // Create a new message with the updated availability time
        let delayed_message = Message { available_at, ..message };

        // Lock the queue and push the message
        let mut queue = self.messages.lock().await;
        queue.push(delayed_message.clone());
        println!("Message pushed: {:?}", delayed_message);

        Ok(())
    }

    /// Removes and returns the highest priority message from the queue that is available for processing.
    ///
    /// Messages that are not yet available due to a delay are not returned.
    ///
    /// Returns `None` if the queue is empty or if no messages are currently available.
    ///
    /// # Errors
    ///
    /// Returns `QueueError::LockError` if the queue lock cannot be acquired.
    ///
    /// # Examples
    ///
    ///
    /// use hexboltmq::queue::{Queue, Message};
    /// use tokio::time::Duration;
    /// let queue = Queue::new();
    /// queue.push(Message { id: 1, content: String::from("Hello"), priority: 5 }, Duration::from_secs(0)).await.unwrap();
    /// let msg = queue.pop().await.unwrap();
    /// assert_eq!(msg.unwrap().priority, 5);
    ///
    pub async fn pop(&self) -> Result<Option<Message>, QueueError> {
        let mut queue = self.messages.lock().await;

        // Check if the top message is available for processing
        if let Some(top_message) = queue.peek() {
            if top_message.available_at <= Instant::now() {
                let msg = queue.pop();
                if let Some(ref m) = msg {
                    println!("Message popped: {:?}", m);
                }
                return Ok(msg);
            }
        }

        // Return None if no messages are available for processing
        Ok(None)
    }

    /// Returns the current size of the queue.
    ///
    /// # Errors
    ///
    /// Returns `QueueError::LockError` if the queue lock cannot be acquired.
    ///
    /// # Examples
    ///
    ///
    /// use hexboltmq::queue::{Queue, Message};
    /// let queue = Queue::new();
    /// assert_eq!(queue.size().await.unwrap(), 0);
    /// queue.push(Message { id: 1, content: String::from("Hello"), priority: 5 }, Duration::from_secs(0)).await.unwrap();
    /// assert_eq!(queue.size().await.unwrap(), 1);
    ///
    pub async fn size(&self) -> Result<usize, QueueError> {
        let queue = self.messages.lock().await;
        Ok(queue.len())
    }
}
