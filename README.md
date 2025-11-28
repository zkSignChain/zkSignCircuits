# zkSign Monorepo
![zkSign banner](public/banner.jpg) 

Hybrid Rust + TypeScript monorepo for zkSign — high-performance cryptography, zk circuits, Anchor programs for Solana, and multi-environment SDKs for Node and browser.

This repository is a production-ready scaffold for teams building ZK-enabled backends and SDKs. It combines a Cargo workspace for Rust crates and a pnpm/Turborepo workspace for TypeScript packages. The goal is reproducible builds and a developer-friendly DX for both Rust and JS ecosystems.

**Quick Start**

Prerequisites:
- **Node.js** v18 or later and `pnpm` (recommended)
- **Rust** (stable) and `rustup`
- Optional but required for WASM builds: `wasm-pack` and `wasm-bindgen-cli`

Install dependencies (Node + Cargo fetch):

```powershell
pnpm install
pnpm run install:all
```

Build everything (Rust → WASM → TypeScript):

```powershell
# Build Rust crates (release)
pnpm -w build:rust

# Build wasm pkg for TypeScript integration (in the zksign-wasm crate)
cd cargo-workspace\crates\zksign-wasm
wasm-pack build --release --target bundler
wasm-pack build --release --target nodejs
cd ..\..\..

# Copy wasm pkg into the TS package and build TS packages
.\scripts\copy-wasm.ps1
pnpm -w build:ts
```

Run all tests (TS + Rust):

```powershell
pnpm run test
```

**Repository Layout**

- `cargo-workspace/` : Cargo workspace containing Rust crates
	- `crates/zksign-crypto/` — crypto utilities (SHA256, Blake2s, Merkle helper)
	- `crates/zksign-circuits/` — circuit harness (mock prover) — intended for Halo2/arkworks integration
	- `crates/zksign-wasm/` — wasm-bindgen bindings exposing prove/verify to JS
	- `crates/zksign-anchor/` — Anchor program skeleton for Solana attestation

- `packages/` : TypeScript workspace managed by `pnpm` and `turbo`
	- `@zksign/core` — shared types and helpers
	- `@zksign/zk-prover` — WASM loader + Node worker helpers (exposes `generateProof` API)
	- `@zksign/zk-verifier` — JS verifier helpers and on-chain integration helpers
	- `@zksign/solana` — Anchor client utilities, PDA helpers, and tx builders
	- `@zksign/client` — HTTP client utilities for submitting proofs
	- `@zksign/cli` — developer CLI for common tasks (init, generate-proof, build-wasm)

- `examples/` : runnable examples
	- `nodejs/` — Node example using `@zksign/zk-prover`
	- `browser/` — minimal browser example (loads WASM via bundler/worker)
	- `solana-demo/` — Anchor.localnet example with deployment helper

- `docs/` : documentation skeleton (architecture, publishing guide, contributing)
- `.github/workflows/ci.yml` : GitHub Actions CI (builds Rust + wasm + runs TS tests)

**How Rust and TypeScript integrate**

- The `zksign-wasm` crate produces a `pkg/` folder via `wasm-pack` containing JS glue and a `.wasm` file. The `packages/zk-prover` package imports that output (copied into `packages/zk-prover/wasm`) and exposes a friendly `generateProof` API.
- Use feature flags in Rust crates for optional heavy dependencies (e.g., Halo2). The scaffold keeps heavy deps optional to remain fast for iteration.

**Key APIs (examples)**

- Generate a proof (TypeScript):

```ts
import { generateProof } from '@zksign/zk-prover';
const proof = await generateProof({ value: 42 });
```

- Verify a proof (TypeScript stub):

```ts
import { verifyProof } from '@zksign/zk-verifier';
const res = await verifyProof(proof);
```

**Developer Notes & Best Practices**

- WASM build: run `wasm-pack build --target bundler` for browser bundlers and `--target nodejs` to support Node consumptions. After building, copy `pkg/` into `packages/zk-prover/wasm` (helper: `scripts/copy-wasm.*`).
- Anchor: `cargo build` for programs and `anchor build` / `anchor deploy` for deployment. `examples/solana-demo/Anchor.toml` is configured for localnet.
- Add heavy circuit deps (Halo2 / arkworks) behind Cargo feature flags in `zksign-circuits`.
- Use `changesets` for TypeScript package versioning and publishing; follow `docs/publishing.md`.

**CI & Releases**

- CI (`.github/workflows/ci.yml`) installs Node and pnpm, sets up Rust, adds `wasm32-unknown-unknown` target, builds Rust, runs `wasm-pack` for the WASM crate, runs Turbo tests and `cargo test`.
- Publishing: configure GitHub secrets for `NPM_TOKEN` and `CARGO_REGISTRY_TOKEN` and add a manual/pipeline workflow for publishing packages (not included here to avoid leaking secrets).

**Local Example: Node**

Build and run the Node example:

```powershell
cd examples\nodejs
pnpm install
pnpm run build
pnpm start
```

Expected: a console log showing that a proof was generated and its length.

**Troubleshooting**

- If `wasm-pack` is not found: install with `cargo install wasm-pack` or use your package manager.
- If Cargo build fails for Anchor: ensure `anchor-cli` and Solana tools are installed and that `rustup` has the correct toolchain.
- If TS packages can’t find the wasm artifact, run the wasm build then `scripts/copy-wasm.ps1` (Windows) or `scripts/copy-wasm.sh` (Unix).

**Contributing & Security**

- See `CONTRIBUTING.md` for contribution guidelines and `SECURITY.md` for vulnerability reporting.
- Use conventional commits and the `changesets` workflow for publishing releases.

**Extending the Monorepo**

- Add new Rust crates under `cargo-workspace/crates/` and add them to `cargo-workspace/Cargo.toml` `members`.
- Add new TS packages under `packages/` and they will be picked up by `pnpm` workspace.

**Files of Interest**

- `cargo-workspace/Cargo.toml` — Cargo workspace definition
- `packages/zk-prover/` — TS wrapper that consumes WASM
- `scripts/copy-wasm.*` — helper to copy WASM artifacts into TS package
- `.github/workflows/ci.yml` — CI pipeline

If you'd like, I can now:
- wire `changesets` + a publish workflow, or
- add a real Halo2 example circuit (note: heavy dependencies), or
- add Husky + Commitlint pre-commit hooks for conventional commits.

— End of README

