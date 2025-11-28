import { Connection, Transaction, Signer, sendAndConfirmTransaction } from '@solana/web3.js';

export async function sendTxWithRetry(connection: Connection, tx: Transaction, signers: Signer[], retries = 3) {
  for (let i = 0; i < retries; i++) {
    try {
      return await sendAndConfirmTransaction(connection, tx, signers, { commitment: 'confirmed' });
    } catch (err) {
      if (i === retries - 1) throw err;
    }
  }
}

export * from './idl-types';
