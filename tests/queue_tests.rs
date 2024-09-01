use hexboltmq::queue::{Queue, Message, QueueError};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_queue_push_and_pop() -> Result<(), QueueError> {
    // Create a new queue
    let queue = Queue::new();

    // Create a message
    let msg1 = Message::new(1, "Test message 1".to_string(), 1, None);

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
async fn test_queue_size() -> Result<(), QueueError> {
    // Create a new queue
    let queue = Queue::new();

    // Queue should be empty initially
    assert_eq!(queue.size().await?, 0);

    // Add a message and check size
    queue.push(Message::new(1, "Message 1".to_string(), 1, None), Duration::from_secs(0)).await?;
    assert_eq!(queue.size().await?, 1);

    Ok(())
}

#[tokio::test]
async fn test_priority_queue_push_and_pop() -> Result<(), QueueError> {
    // Create a new queue
    let queue = Queue::new();

    // Create messages with different priorities
    let msg1 = Message::new(1, "Low priority message".to_string(), 1, None);
    let msg2 = Message::new(2, "High priority message".to_string(), 10, None);
    let msg3 = Message::new(3, "Medium priority message".to_string(), 5, None);

    // Push messages to the queue
    queue.push(msg1.clone(), Duration::from_secs(0)).await?;
    queue.push(msg2.clone(), Duration::from_secs(0)).await?;
    queue.push(msg3.clone(), Duration::from_secs(0)).await?;

    // Pop messages and verify order by priority
    let popped_msg1 = queue.pop().await?;
    assert_eq!(popped_msg1, Some(msg2)); // High priority

    let popped_msg2 = queue.pop().await?;
    assert_eq!(popped_msg2, Some(msg3)); // Medium priority

    let popped_msg3 = queue.pop().await?;
    assert_eq!(popped_msg3, Some(msg1)); // Low priority

    Ok(())
}

#[tokio::test]
async fn test_delayed_message_push_and_pop() -> Result<(), QueueError> {
    // Create a new queue
    let queue = Queue::new();

    // Create a message with a 2-second delay
    let msg = Message::new(1, "Delayed message".to_string(), 1, Some(Duration::from_secs(2)));

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
    let msg1 = Message::new(1, "Message 1".to_string(), 1, Some(Duration::from_secs(1))); // Delayed by 1 second
    let msg2 = Message::new(2, "Message 2".to_string(), 5, None); // Available immediately
    let msg3 = Message::new(3, "Message 3".to_string(), 10, Some(Duration::from_secs(2))); // Delayed by 2 seconds

    // Push messages to the queue with different delays
    queue.push(msg1.clone(), Duration::from_secs(1)).await?;
    queue.push(msg2.clone(), Duration::from_secs(0)).await?;
    queue.push(msg3.clone(), Duration::from_secs(2)).await?;

    // Immediately attempt to pop a batch (should only get msg2)
    let batch = queue.pop_batch(3).await?;
    assert_eq!(batch.len(), 1);
    assert_eq!(batch[0].id, msg2.id); // msg2 should be popped first because it's available immediately

    // Wait for the delays to pass
    sleep(Duration::from_secs(2)).await;

    // Now all messages should be available
    let batch = queue.pop_batch(3).await?;
    assert_eq!(batch.len(), 2);
    assert_eq!(batch[0].id, msg3.id); // Highest priority and now available
    assert_eq!(batch[1].id, msg1.id); // Next in line

    Ok(())
}
