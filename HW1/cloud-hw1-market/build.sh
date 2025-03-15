#!/bin/bash

# 停止執行發生錯誤的指令
set -e

echo "Building project..."
cargo build --release

echo "Build complete! Executable is located at target/release/cloud-hw1-market"
