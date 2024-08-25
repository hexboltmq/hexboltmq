mod core;

use core::{Queue, Message};

fn main() {
    let mut queue = Queue::new("example_queue");
    println!("Queue name: {}", queue.name);

    let message = Message {
        id: "1".to_string(),
        body: b"Hello, HexboltMQ!".to_vec(),
    };
    queue.enqueue(message);
    println!("Message count: {}", queue.get_message_count());
    println!("Dequeued message: {:?}", queue.dequeue());
    println!("Message count: {}", queue.get_message_count());
}
