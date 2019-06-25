# Mimalloc Rust
[![Build Status](https://travis-ci.org/purpleprotocol/mimalloc_rust.svg?branch=master)](https://travis-ci.org/purpleprotocol/mimalloc_rust) [![Latest Version]][crates.io] [![Documentation]][docs.rs]

A drop-in global allocator wrapper around the [mimalloc](https://github.com/microsoft/mimalloc) allocator.
Mimalloc is a general purpose, performance oriented allocator built by Microsoft.

## Usage
```rust
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

## Usage without secure mode
By default this library builds mimalloc in secure mode. This means that
heap allocations are encrypted, but this results in a 3% increase in overhead.

To disable secure mode, in `Cargo.toml`:
```rust
[dependencies]
mimalloc = { version = "*", default-features = false }
```

[crates.io]: https://crates.io/crates/mimalloc
[Latest Version]: https://img.shields.io/crates/v/mimalloc.svg
[Documentation]: https://docs.rs/mimalloc/badge.svg
[docs.rs]: https://docs.rs/mimalloc