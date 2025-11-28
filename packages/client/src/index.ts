import got from 'got';

export async function postProof(endpoint: string, proof: Uint8Array) {
  const res = await got.post(endpoint, {
    json: { proof: Buffer.from(proof).toString('hex') },
    responseType: 'json'
  });
  return res.body;
}
