#!/bin/bash

# Exit on error
set -e

# Ensure we have the wasm32 target
rustup target add wasm32-unknown-unknown

# Build the contract
cargo build --target wasm32-unknown-unknown --release
