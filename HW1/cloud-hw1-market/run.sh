#!/bin/bash

# 停止執行發生錯誤的指令
set -e

# 確保執行檔存在
if [ ! -f target/release/cloud-hw1-market ]; then
    echo "Error: Executable not found. Please run ./build.sh first."
    exit 1
fi

echo "Running cloud-hw1-market CLI..."
target/release/cloud-hw1-market
