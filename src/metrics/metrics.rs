use metrics::{register_counter, register_gauge, increment_counter, gauge};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Metrics to track the queue performance.
#[derive(Debug, Clone)]
pub struct Metrics {
    pub produced_messages: Arc<Mutex<u64>>,
    pub consumed_messages: Arc<Mutex<u64>>,
    pub failed_messages: Arc<Mutex<u64>>,
    pub retry_attempts: Arc<Mutex<u64>>,
    pub queue_size: Arc<Mutex<u64>>,
}

impl Metrics {
    /// Initializes the metrics system and exposes them through Prometheus.
    pub fn init() -> PrometheusHandle {
        // Register metrics
        register_counter!("produced_messages_total");
        register_counter!("consumed_messages_total");
        register_counter!("failed_messages_total");
        register_counter!("retry_attempts_total");
        register_gauge!("queue_size");

        // Initialize Prometheus exporter
        let builder = PrometheusBuilder::new();
        builder.install_recorder().unwrap()
    }

    /// Increments the produced messages counter.
    pub async fn increment_produced(&self) {
        increment_counter!("produced_messages_total");
        let mut produced = self.produced_messages.lock().await;
        *produced += 1;
    }

    /// Increments the consumed messages counter.
    pub async fn increment_consumed(&self) {
        increment_counter!("consumed_messages_total");
        let mut consumed = self.consumed_messages.lock().await;
        *consumed += 1;
    }

    /// Increments the failed messages counter.
    pub async fn increment_failed(&self) {
        increment_counter!("failed_messages_total");
        let mut failed = self.failed_messages.lock().await;
        *failed += 1;
    }

    /// Increments the retry attempts counter.
    pub async fn increment_retry(&self) {
        increment_counter!("retry_attempts_total");
        let mut retries = self.retry_attempts.lock().await;
        *retries += 1;
    }

    /// Sets the queue size gauge.
    pub async fn set_queue_size(&self, size: u64) {
        gauge!("queue_size", size as f64);
        let mut queue_size = self.queue_size.lock().await;
        *queue_size = size;
    }
}
