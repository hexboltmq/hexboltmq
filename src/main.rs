// src/main.rs
mod queue;
mod producer;
mod consumer;
mod network;
mod storage;
mod config;
mod auth;
mod scheduler;
mod metrics;
mod cli;
mod utils;
mod cluster;
mod logging;
mod plugins;

use log::info;
use crate::queue::{Queue, Message};

fn main() {
    env_logger::init();
    info!("HexboltMQ is starting...");

    // Create a new queue
    let queue = Queue::new();

    // Push messages to the queue
    queue.push(Message { id: 1, content: "Hello, HexboltMQ!".to_string(), priority: 1 });
    queue.push(Message { id: 2, content: "Another message".to_string(), priority: 2 });

    // Pop messages from the queue
    if let Some(message) = queue.pop() {
        println!("Popped message: {:?}", message);
    }

    if let Some(message) = queue.pop() {
        println!("Popped message: {:?}", message);
    }

    // Check the size of the queue
    println!("Current queue size: {}", queue.size());
}
