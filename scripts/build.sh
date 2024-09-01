#!/bin/bash

# Build the project in release mode
echo "Building Hexboltmq..."
cargo build --release

echo "Build complete. Binary is located in target/release/hexboltmq"
