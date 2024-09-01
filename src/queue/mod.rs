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

/// Custom errors that can occur when interacting with the queue.
#[derive(Debug)]
pub enum QueueError {
    /// Error occurring when a lock cannot be acquired (not applicable for Tokio Mutex).
    LockError,
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
    /// use tokio_test::block_on;
    ///
    /// let queue = Queue::new();
    /// block_on(async {
    ///     queue.push(Message { id: 1, content: String::from("Hello"), priority: 5 }).await.unwrap();
    /// });
    /// ```
    pub async fn push(&self, message: Message) -> Result<(), QueueError> {
        let mut queue = self.messages.lock().await;
        queue.push(message.clone());
        println!("Message pushed: {:?}", message);
        Ok(())
    }

    /// Removes and returns the highest priority message from the queue.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexboltmq::queue::{Queue, Message};
    /// use tokio_test::block_on;
    ///
    /// let queue = Queue::new();
    /// block_on(async {
    ///     queue.push(Message { id: 1, content: String::from("Hello"), priority: 5 }).await.unwrap();
    ///     let msg = queue.pop().await.unwrap();
    ///     assert_eq!(msg.unwrap().priority, 5);
    /// });
    /// ```
    pub async fn pop(&self) -> Result<Option<Message>, QueueError> {
        let mut queue = self.messages.lock().await;
        let msg = queue.pop();
        if let Some(ref m) = msg {
            println!("Message popped: {:?}", m);
        }
        Ok(msg)
    }

    /// Returns the current size of the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use hexboltmq::queue::{Queue, Message};
    /// use tokio_test::block_on;
    ///
    /// let queue = Queue::new();
    /// block_on(async {
    ///     assert_eq!(queue.size().await.unwrap(), 0);
    ///     queue.push(Message { id: 1, content: String::from("Hello"), priority: 5 }).await.unwrap();
    ///     assert_eq!(queue.size().await.unwrap(), 1);
    /// });
    /// ```
    pub async fn size(&self) -> Result<usize, QueueError> {
        let queue = self.messages.lock().await;
        Ok(queue.len())
    }
}
