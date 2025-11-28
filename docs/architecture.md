# Architecture Overview

This document explains the hybrid monorepo architecture connecting Rust crates with TypeScript packages.

- Rust crates live under `cargo-workspace/crates` and include high-performance code and WASM bindings.
- TypeScript packages live under `packages/` and depend on WASM artifacts produced by Rust.
- `examples/` demonstrate Node, browser, and Solana integration.

See README.md for quickstart.
