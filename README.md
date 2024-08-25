# HexboltMQ

**HexboltMQ** is a distributed message queueing framework designed for modern-day workflows. It focuses on delivering durable, fault-tolerant, highly performant, and highly available message queues. HexboltMQ is built to support a wide range of advanced messaging patterns, including work queues, priority queues, fan-out, broadcast, and more. With strong consistency guarantees, compression, and poison pill detection, HexboltMQ is an ideal choice for building reliable and scalable systems.

## Features

- **Durable and Fault-Tolerant**: Ensure messages are never lost with persistent storage and replication across multiple nodes.
- **High Performance**: Leverage asynchronous processing, efficient data structures, and optimized I/O for low-latency message handling.
- **High Availability**: Clustering and load balancing ensure that your message queues are always available, even during node failures.
- **Advanced Routing**: Supports various message routing strategies, including work queues, priority queues, fan-out, and broadcast.
- **Compression**: Built-in support for message compression to save bandwidth and storage.
- **Strong Consistency**: Implements consensus algorithms like Raft to maintain consistent state across distributed systems.
- **Poison Pill Detection**: Automatically detect and handle problematic messages to prevent them from disrupting the system.

## Getting Started

### Prerequisites

- **Rust**: Ensure that you have the latest stable version of Rust installed. You can install Rust using [rustup](https://rustup.rs/).
- **Docker**: (Optional) For running HexboltMQ in a containerized environment.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/adi3g/hexboltmq.git
   cd hexboltmq
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the Hexboltmq server:

   ```bash
   cargo run --bin hexboltmq-server
   ```

### Usage

HexboltMQ provides both gRPC and REST APIs for interacting with the message queues.

- **Enqueue a Message**: Send a message to a queue.
- **Dequeue a Message**: Receive a message from a queue.
- **Acknowledge a Message**: Confirm the processing of a message.

Refer to the API documentation for detailed usage instructions.

## Architecture Overview

HexboltMQ's architecture is designed for scalability and fault tolerance:

- **Core**: Handles the main message queueing logic, including message routing, prioritization, and processing.
- **Storage**: Manages both in-memory and persistent storage for queues, ensuring durability.
- **Cluster**: Implements clustering and replication to provide high availability and fault tolerance.
- **API**: Exposes gRPC and REST APIs for easy integration with other systems.
- **Utilities**: Includes utilities for compression, monitoring, and logging.

## Contributing

Contributions are welcome! If you'd like to contribute to HexboltMQ, please follow these steps:

1. Fork the repository.
2. Create a new branch with a descriptive name.
3. Make your changes and add tests if necessary.
4. Submit a pull request with a detailed description of your changes.

Please read our [contributing guidelines](CONTRIBUTING.md) for more information.

## License

HexboltMQ is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or suggestions, feel free to open an issue or contact the maintainers at `contact@adib-grouz.com`.
