// tests/test_queue.rs
use hexboltmq::queue::{Queue, Message};

#[test]
fn test_queue_push_and_pop() {
    // Create a new queue
    let queue = Queue::new();

    // Create a message
    let msg1 = Message { id: 1, content: "Test message 1".to_string(), priority: 1 };
    let msg2 = Message { id: 2, content: "Test message 2".to_string(), priority: 2 };

    // Push messages to the queue
    queue.push(msg1.clone());
    queue.push(msg2.clone());

    // Check the size of the queue
    assert_eq!(queue.size(), 2);

    // Pop messages and verify content
    let popped_msg1 = queue.pop();
    assert_eq!(popped_msg1, Some(msg1));

    let popped_msg2 = queue.pop();
    assert_eq!(popped_msg2, Some(msg2));

    // The queue should be empty now
    assert_eq!(queue.size(), 0);
}

#[test]
fn test_queue_empty_pop() {
    // Create a new queue
    let queue = Queue::new();

    // Pop from an empty queue should return None
    let popped_msg = queue.pop();
    assert_eq!(popped_msg, None);
}

#[test]
fn test_queue_size() {
    // Create a new queue
    let queue = Queue::new();

    // Queue should be empty initially
    assert_eq!(queue.size(), 0);

    // Add messages and check size
    queue.push(Message { id: 1, content: "Message 1".to_string(), priority: 1 });
    assert_eq!(queue.size(), 1);

    queue.push(Message { id: 2, content: "Message 2".to_string(), priority: 2 });
    assert_eq!(queue.size(), 2);

    // Pop a message and check size
    queue.pop();
    assert_eq!(queue.size(), 1);
}
