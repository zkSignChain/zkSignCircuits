import { GenerateProofInput, Proof } from '@zksign/core';

// Note: `wasm` artifacts are expected to be copied from `cargo-workspace/crates/zksign-wasm/pkg`
// into `packages/zk-prover/wasm` by the root `build:wasm` script.

export async function generateProof(input: GenerateProofInput): Promise<Proof> {
  // If running in Node and wasm pkg exists, load it
  try {
    // dynamic import of the wasm glue code
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const wasm = await import('../wasm');
    const inputJson = JSON.stringify({ value: input.value });
    const proofBytes: Uint8Array = wasm.wasm_generate_proof(inputJson);
    return proofBytes;
  } catch (err) {
    // Fallback: call a remote service or throw
    throw new Error('WASM not available, build the wasm package first');
  }
}
