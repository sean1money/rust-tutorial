#!/bin/bash

# Add timestamp function
timestamp() {
  date +"%Y-%m-%d %H:%M:%S"
}

echo "[$(timestamp)] Starting build process..."
cargo build --workspace --release

echo "[$(timestamp)] Running test utilities..."
./target/release/om-test-utils

echo "[$(timestamp)] Cleaning up previous node processes..."
# kill all process of 1m-node
killall 1m-node || true

echo "[$(timestamp)] shutting down 1M testnet..."