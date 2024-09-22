use tokio::time::{sleep, Duration, Interval};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a scheduler that can handle delayed and periodic tasks.
pub struct Scheduler {
    retry_interval: Duration,          // Interval between retries for failed messages
    cleanup_interval: Duration,        // Interval for periodic cleanup tasks
}

impl Scheduler {
    /// Create a new scheduler with the given intervals.
    ///
    /// # Arguments
    ///
    /// * `retry_interval` - The interval between retries for failed messages.
    /// * `cleanup_interval` - The interval at which to run cleanup tasks.
    pub fn new(retry_interval: Duration, cleanup_interval: Duration) -> Self {
        Scheduler {
            retry_interval,
            cleanup_interval,
        }
    }

    /// Schedule a task to be executed after a specified delay.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to be executed.
    /// * `delay` - The delay after which the task should be executed.
    pub async fn schedule_delayed<F>(&self, task: F, delay: Duration)
    where
        F: FnOnce() + Send + 'static,
    {
        // Sleep for the given duration
        sleep(delay).await;
        // Execute the task
        tokio::spawn(async move {
            task();
        });
    }

    /// Schedule a task to be retried after a delay (retry logic).
    ///
    /// # Arguments
    ///
    /// * `task` - The task to be retried.
    pub async fn schedule_retry<F>(&self, task: F)
    where
        F: Fn() + Send + 'static + Clone,
    {
        let task_clone = task.clone(); // Clone the closure
        loop {
            // Sleep for the retry interval
            sleep(self.retry_interval).await;

            // Clone the task for the next iteration
            let task_instance = task_clone.clone();

            // Retry the task
            tokio::spawn(async move {
                task_instance();
            });
        }
    }

    /// Start a periodic cleanup task that runs every `cleanup_interval`.
    ///
    /// # Arguments
    ///
    /// * `cleanup_task` - The periodic cleanup task to be executed.
    pub async fn start_periodic_cleanup<F>(&self, cleanup_task: F)
    where
        F: Fn() + Send + 'static + Clone,
    {
        let cleanup_task_clone = cleanup_task.clone(); // Clone the closure
        let mut interval = tokio::time::interval(self.cleanup_interval);
        loop {
            interval.tick().await;

            // Clone the cleanup task for the next iteration
            let cleanup_task_instance = cleanup_task_clone.clone();

            tokio::spawn(async move {
                cleanup_task_instance();
            });
        }
    }
}
