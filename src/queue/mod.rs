// src/queue/mod.rs

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tokio::sync::Mutex;
use std::sync::Arc;

/// A message that can be added to the queue.
///
/// Each message has an ID, content, and a priority. The priority determines
/// the order in which messages are processed, with higher values being processed first.
#[derive(Debug, Clone, Eq)]
pub struct Message {
    /// The unique identifier of the message.
    pub id: u64,
    /// The content of the message.
    pub content: String,
    /// The priority of the message. Higher values indicate higher priority.
    pub priority: u8,
}

// Implement ordering for the message to be used in a priority queue.
impl Ord for Message {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Higher priority messages come first
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

/// A thread-safe priority queue for managing `Message` objects.
///
/// The `Queue` allows multiple producers and consumers to safely
/// push and pop messages concurrently, with messages being processed
/// in order of their priority.
#[derive(Debug, Clone)]
pub struct Queue {
    messages: Arc<Mutex<BinaryHeap<Message>>>,
}

impl Queue {
    /// Creates a new, empty `Queue`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexboltmq::queue::Queue;
    /// let queue = Queue::new();
    /// ```
    pub fn new() -> Self {
        Queue {
            messages: Arc::new(Mutex::new(BinaryHeap::new())),
        }
    }

    /// Adds a message to the queue.
    ///
    /// Messages are stored based on their priority, with higher priority messages
    /// being processed first.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to add to the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexboltmq::queue::{Queue, Message};
    /// let queue = Queue::new();
    /// queue.push(Message { id: 1, content: String::from("Hello"), priority: 5 }).await;
    /// ```
    pub async fn push(&self, message: Message) {
        let mut queue = self.messages.lock().await;
        queue.push(message.clone());
        println!("Message pushed: {:?}", message);
    }

    /// Removes and returns the highest priority message from the queue.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexboltmq::queue::{Queue, Message};
    /// let queue = Queue::new();
    /// queue.push(Message { id: 1, content: String::from("Hello"), priority: 5 }).await;
    /// let msg = queue.pop().await;
    /// assert_eq!(msg.unwrap().priority, 5);
    /// ```
    pub async fn pop(&self) -> Option<Message> {
        let mut queue = self.messages.lock().await;
        let msg = queue.pop();
        if let Some(ref m) = msg {
            println!("Message popped: {:?}", m);
        }
        msg
    }

    /// Returns the current size of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexboltmq::queue::{Queue, Message};
    /// let queue = Queue::new();
    /// assert_eq!(queue.size().await, 0);
    /// queue.push(Message { id: 1, content: String::from("Hello"), priority: 5 }).await;
    /// assert_eq!(queue.size().await, 1);
    /// ```
    pub async fn size(&self) -> usize {
        let queue = self.messages.lock().await;
        queue.len()
    }
}
