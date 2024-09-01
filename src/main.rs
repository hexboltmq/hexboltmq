//! # HexboltMQ
//!
//! HexboltMQ is a message queue system implemented in Rust with priority handling,
//! concurrency support, and async capabilities using Tokio.
//!
//! ## Modules
//!
//! - `queue`: Core queue functionality with priority handling.
//! - `producer`: Module for managing message producers.
//! - `consumer`: Module for managing message consumers.
//!

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

fn main() {
    env_logger::init();
    info!("HexboltMQ is starting...");
}
