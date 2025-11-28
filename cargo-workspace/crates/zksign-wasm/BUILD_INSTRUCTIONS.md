WASM build instructions

Requirements:
- `wasm-pack` (install with `cargo install wasm-pack`)
- `wasm-bindgen-cli` (optional for additional processing)

Build (bundler target + node target):

```powershell
cd cargo-workspace/crates/zksign-wasm
wasm-pack build --release --target bundler
wasm-pack build --release --target nodejs
```

Then copy `pkg/` to `packages/zk-prover/wasm` for the TypeScript package to import.
