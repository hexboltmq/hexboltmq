use hexboltmq::queue::{Queue, Message, QueueError};
use tokio::time::{sleep, Duration, Instant};

#[tokio::test]
async fn test_queue_push_and_pop() -> Result<(), QueueError> {
    // Create a new queue
    let queue = Queue::new();

    // Create a message with no delay
    let msg1 = Message {
        id: 1,
        content: "Test message 1".to_string(),
        priority: 1,
        available_at: Instant::now(), // Available immediately
        retry_count: 0,
        max_retries: 3,
    };

    // Push a message to the queue
    queue.push(msg1.clone(), Duration::from_secs(0)).await?;

    // Pop the message from the queue
    let popped_msg = queue.pop().await?;
    assert_eq!(popped_msg, Some(msg1));

    Ok(())
}

#[tokio::test]
async fn test_queue_empty_pop() -> Result<(), QueueError> {
    // Create a new queue
    let queue = Queue::new();

    // Pop from an empty queue should return None
    let popped_msg = queue.pop().await?;
    assert_eq!(popped_msg, None);

    Ok(())
}

#[tokio::test]
async fn test_delayed_message_push_and_pop() -> Result<(), QueueError> {
    // Create a new queue
    let queue = Queue::new();

    // Create a message with a 2-second delay
    let msg = Message {
        id: 1,
        content: "Delayed message".to_string(),
        priority: 1,
        available_at: Instant::now() + Duration::from_secs(2), // Delayed availability
        retry_count: 0,
        max_retries: 3,
    };

    // Push the message to the queue with a 2-second delay
    queue.push(msg.clone(), Duration::from_secs(2)).await?;

    // Immediately attempt to pop (should get None because of delay)
    assert!(queue.pop().await?.is_none());

    // Wait for the delay to pass
    sleep(Duration::from_secs(2)).await;

    // Now the message should be available
    let popped_msg = queue.pop().await?;
    assert_eq!(popped_msg, Some(msg));

    Ok(())
}


#[tokio::test]
async fn test_batch_processing() -> Result<(), QueueError> {
    // Create a new queue
    let queue = Queue::new();

    // Create messages with varying delays
    let msg1 = Message {
        id: 1,
        content: "Message 1".to_string(),
        priority: 1,
        available_at: Instant::now() + Duration::from_secs(1),
        retry_count: 0,
        max_retries: 3,
    };
    let msg2 = Message {
        id: 2,
        content: "Message 2".to_string(),
        priority: 5,
        available_at: Instant::now(), // Available immediately
        retry_count: 0,
        max_retries: 3,
    };
    let msg3 = Message {
        id: 3,
        content: "Message 3".to_string(),
        priority: 10,
        available_at: Instant::now() + Duration::from_secs(2),
        retry_count: 0,
        max_retries: 3,
    };

    // Push messages to the queue
    queue.push(msg1.clone(), Duration::from_secs(1)).await?;
    queue.push(msg2.clone(), Duration::from_secs(0)).await?;
    queue.push(msg3.clone(), Duration::from_secs(2)).await?;

    // Wait briefly
    sleep(Duration::from_millis(50)).await;

    // Pop a batch
    println!("Attempting first batch pop:");
    let batch = queue.pop_batch(3).await?;
    assert_eq!(batch.len(), 1); // Should have only msg2 available immediately
    assert_eq!(batch[0].id, msg2.id);

    // Wait for 1 second; now msg1 should also be available
    sleep(Duration::from_secs(1)).await;

    // Next batch
    println!("Attempting second batch pop:");
    let batch = queue.pop_batch(3).await?;
    assert_eq!(batch.len(), 1);
    assert_eq!(batch[0].id, msg1.id);

    // Wait for another 1 second; now msg3 should also be available
    sleep(Duration::from_secs(1)).await;

    // Final batch
    println!("Attempting third batch pop:");
    let batch = queue.pop_batch(3).await?;
    assert_eq!(batch.len(), 1);
    assert_eq!(batch[0].id, msg3.id);

    Ok(())
}

#[tokio::test]
async fn test_message_acknowledgment_and_retries() {
    let queue = Queue::new();

    // Add a message with retry capabilities
    let message = Message {
        id: 1,
        content: String::from("Retry message"),
        priority: 5,
        available_at: Instant::now(),
        retry_count: 0,
        max_retries: 3,
    };

    queue.push(message.clone(), Duration::from_secs(0)).await.unwrap();

    // Pop and simulate failure, causing a retry
    queue.pop().await.unwrap();

    // Verify that the message has been retried (check logs or state)
    // Add assertions as needed to validate retry behavior
}