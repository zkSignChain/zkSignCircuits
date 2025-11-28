import { generateProof } from '@zksign/zk-prover';

async function main() {
  console.log('Generating proof for value 7...');
  const proof = await generateProof({ value: 7 });
  console.log('Proof length', proof.length);
}

main().catch((e) => {
  console.error(e);
});
