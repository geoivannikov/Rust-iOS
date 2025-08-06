#!/bin/bash

set -e

OUT_DIR=target/xcframework
HEADER_OUT=bridge.h
TEMP_DIR=target/xcframework_temp

echo "Running tests..."
cargo test
echo "Tests passed."

echo "Cleaning target before building XCFramework..."
rm -rf $OUT_DIR
rm -rf $TEMP_DIR

echo "Building for aarch64-apple-ios (device)..."
cargo build --release --target aarch64-apple-ios

echo "Building for aarch64-apple-ios-sim (simulator)..."
cargo build --release --target aarch64-apple-ios-sim

echo "Generating C header..."
cbindgen --output $HEADER_OUT

echo "Preparing temporary dirs..."
mkdir -p $TEMP_DIR/device
mkdir -p $TEMP_DIR/sim

cp target/aarch64-apple-ios/release/librust_core.a $TEMP_DIR/device/
cp target/aarch64-apple-ios-sim/release/librust_core.a $TEMP_DIR/sim/
cp bridge.h $TEMP_DIR/device/
cp bridge.h $TEMP_DIR/sim/

echo "Creating XCFramework..."
xcodebuild -create-xcframework \
  -library $TEMP_DIR/device/librust_core.a \
  -headers $TEMP_DIR/device \
  -library $TEMP_DIR/sim/librust_core.a \
  -headers $TEMP_DIR/sim \
  -output $OUT_DIR/rust_core.xcframework

echo "Done. Output: $OUT_DIR/rust_core.xcframework + $HEADER_OUT"
