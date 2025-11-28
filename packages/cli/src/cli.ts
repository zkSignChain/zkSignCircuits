#!/usr/bin/env node
import { Command } from 'commander';
import { generateProof } from '@zksign/zk-prover';

const program = new Command();

program
  .name('zksign')
  .description('zksign dev CLI')
  .version('0.0.0');

program
  .command('generate-proof')
  .description('Generate a proof for a sample input')
  .option('-v, --value <n>', 'numeric value', '42')
  .action(async (opts) => {
    const v = Number(opts.value);
    const proof = await generateProof({ value: v });
    console.log('Proof length:', proof.length);
  });

program.parse(process.argv);
