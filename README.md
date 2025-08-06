# Rust-iOS

A minimal Rust-powered password generator, encryptor, and storage backend integrated with a SwiftUI iOS app.

## 🚀 Features

- Generate random passwords using Rust
- Encrypt/decrypt using ChaCha20 + Base64
- Store encrypted passwords in a local embedded database
- Access functionality in Swift via FFI (`.xcframework` and C header)
- Clean SwiftUI UI for interacting with the password logic

## 📦 Build Instructions

Before running the iOS app, build the Rust static library and header:

```bash
./build.sh
```

## 🧪 Testing

To run unit tests for the Rust logic:

```bash
cargo test
```

---
