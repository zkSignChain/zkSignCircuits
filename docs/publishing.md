# Publishing and Release Flow

TypeScript packages:
- Uses `changesets` (recommended). Create changesets, run `pnpm -w changeset version` and `pnpm -w changeset publish`.

Rust crates:
- Use `cargo publish` per-crate. Follow crates.io guidelines and sign via GPG if required.

CI can be extended to package wasm artifacts and create release bundles.
