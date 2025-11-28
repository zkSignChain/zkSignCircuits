/**
 * @zksign/core
 * Minimal TypeScript types and helpers used across packages.
 */

export type Proof = Uint8Array;

export interface GenerateProofInput {
  value: number;
}

export interface VerifierResult {
  ok: boolean;
}

export const toHex = (buf: Uint8Array) => Buffer.from(buf).toString('hex');
