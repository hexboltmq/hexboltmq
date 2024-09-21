use rocksdb::{DB, Options, IteratorMode};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use bincode;
use std::path::Path;

/// Represents a storage system backed by RocksDB for persisting messages.
#[derive(Debug, Clone)]
pub struct Storage {
    db: Arc<Mutex<DB>>,   // Thread-safe access to RocksDB
}

/// Represents a message that will be stored in the queue and the storage system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,            // Unique identifier of the message
    pub content: String,    // Content of the message
    pub priority: u8,       // Message priority
    pub retry_count: u8,    // Retry count
    pub max_retries: u8,    // Max retries allowed
}

impl Storage {
    /// Initializes the RocksDB storage engine at the specified path.
    pub fn new(db_path: &str) -> Self {
        let mut options = Options::default();
        options.create_if_missing(true);

        let db = DB::open(&options, db_path).expect("Failed to open RocksDB");
        Storage {
            db: Arc::new(Mutex::new(db)),
        }
    }

    /// Saves a message to the storage system.
    ///
    /// # Arguments
    /// * `message` - The message to persist.
    pub async fn save_message(&self, message: &Message) -> Result<(), String> {
        let db = self.db.lock().await;

        let key = message.id.to_be_bytes();
        let value = bincode::serialize(&message).map_err(|e| e.to_string())?;

        db.put(key, value).map_err(|e| e.to_string())?;

        println!("Message saved: {:?}", message);
        Ok(())
    }

    /// Loads all messages from the storage system and returns them as a vector.
    pub async fn load_all_messages(&self) -> Result<Vec<Message>, String> {
        let db = self.db.lock().await;
        let mut messages = Vec::new();

        let iter = db.iterator(IteratorMode::Start);
        for item in iter {
            let (_, value) = item.map_err(|e| e.to_string())?;
            let message: Message = bincode::deserialize(&value).map_err(|e| e.to_string())?;
            messages.push(message);
        }

        println!("Loaded {} messages from storage.", messages.len());
        Ok(messages)
    }

    /// Deletes a message from the storage system after it has been acknowledged.
    ///
    /// # Arguments
    /// * `message_id` - The unique identifier of the message to delete.
    pub async fn delete_message(&self, message_id: u64) -> Result<(), String> {
        let db = self.db.lock().await;
        let key = message_id.to_be_bytes();

        db.delete(key).map_err(|e| e.to_string())?;
        println!("Message with ID {} deleted from storage.", message_id);
        Ok(())
    }

    /// Load a specific message by its ID from the storage system.
    ///
    /// # Arguments
    /// * `message_id` - The unique identifier of the message to retrieve.
    pub async fn load_message(&self, message_id: u64) -> Result<Option<Message>, String> {
        let db = self.db.lock().await;
        let key = message_id.to_be_bytes();

        if let Some(value) = db.get(key).map_err(|e| e.to_string())? {
            let message: Message = bincode::deserialize(&value).map_err(|e| e.to_string())?;
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }
}
