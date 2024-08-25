use hexboltmq::core::{Queue, Message};

#[test]
fn test_enqueue_dequeue() {
    let mut queue = Queue::new("test_queue");
    let message = Message {
        id: "1".to_string(),
        body: b"Hello, Hexbolt!".to_vec(),
    };

    queue.enqueue(message);

    // Test that the message is added to the queue
    assert_eq!(queue.get_message_count(), 1);

    // Test that we can dequeue the message
    let dequeued_message = queue.dequeue().unwrap();
    assert_eq!(dequeued_message.id, "1");
    assert_eq!(dequeued_message.body, b"Hello, Hexbolt!".to_vec());
}

#[test]
fn test_queue_empty_after_dequeue() {
    let mut queue = Queue::new("test_queue");
    let message = Message {
        id: "2".to_string(),
        body: b"Another message".to_vec(),
    };

    queue.enqueue(message);
    let _ = queue.dequeue(); // Dequeue the only message

    // Test that the queue is empty after dequeue
    assert_eq!(queue.get_message_count(), 0);
}

#[test]
fn test_multiple_messages() {
    let mut queue = Queue::new("test_queue");

    let message1 = Message {
        id: "1".to_string(),
        body: b"First message".to_vec(),
    };

    let message2 = Message {
        id: "2".to_string(),
        body: b"Second message".to_vec(),
    };

    queue.enqueue(message1);
    queue.enqueue(message2);

    // Test that both messages are in the queue
    assert_eq!(queue.get_message_count(), 2);

    // Dequeue and check order
    let dequeued_message1 = queue.dequeue().unwrap();
    assert_eq!(dequeued_message1.id, "2");

    let dequeued_message2 = queue.dequeue().unwrap();
    assert_eq!(dequeued_message2.id, "1");
}
