#!/bin/bash

PROJECT_NAME="yral-test-backend"
BUILD_TARGET="wasm32-unknown-unknown"
BUILD_DIR="target/$BUILD_TARGET/release"
WASM_FILE="yral_test_backend.wasm"
DID_FILE="$PROJECT_NAME.did"
SRC_DIR="src/$PROJECT_NAME"

if [ -f "$BUILD_DIR/$WASM_FILE" ]; then
  echo "Removing existing WASM file..."
  rm "$BUILD_DIR/$WASM_FILE"
fi

if [ -f "$SRC_DIR/$DID_FILE" ]; then
  echo "Removing existing DID file..."
  rm "$SRC_DIR/$DID_FILE"
fi

echo "Building the project..."
cargo build --release --target "$BUILD_TARGET"

echo "Extracting candid..."
candid-extractor "$BUILD_DIR/$WASM_FILE" > "$SRC_DIR/$DID_FILE"

echo "Candids generated successfully."