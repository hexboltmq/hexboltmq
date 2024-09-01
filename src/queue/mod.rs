// src/queue/mod.rs
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// Define the message structure
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: u64,
    pub content: String,
    pub priority: u8,
}

// Define the queue structure
#[derive(Debug)]
pub struct Queue {
    messages: Arc<Mutex<VecDeque<Message>>>,
}

impl Queue {
    // Create a new, empty queue
    pub fn new() -> Self {
        Queue {
            messages: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    // Add a message to the queue
    pub fn push(&self, message: Message) {
        let mut queue = self.messages.lock().unwrap();
        queue.push_back(message.clone());
        println!("Message pushed: {:?}", message.clone());
    }

    // Remove and return a message from the front of the queue
    pub fn pop(&self) -> Option<Message> {
        let mut queue = self.messages.lock().unwrap();
        let msg = queue.pop_front();
        if let Some(ref m) = msg {
            println!("Message popped: {:?}", m);
        }
        msg
    }

    // Check the current size of the queue
    pub fn size(&self) -> usize {
        let queue = self.messages.lock().unwrap();
        queue.len()
    }
}
