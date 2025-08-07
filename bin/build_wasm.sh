#!/bin/bash

# Exit on any error
set -e

# Build WASM module in release mode
cd agent-template
cargo build --release --target wasm32-unknown-unknown

# Create target directory if it doesn't exist
mkdir -p ../target/wasm32-unknown-unknown/release/

# Copy WASM file to expected location
cp target/wasm32-unknown-unknown/release/uomi_nodes_vault_agent.wasm ../target/wasm32-unknown-unknown/release/uomi_nodesvault_agent.wasm

echo "WASM build and setup completed successfully"