const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { resolve } = require('node:path');

function main() {
  const [mode, bindingPath, inputPath, trustPath] = process.argv.slice(2);
  if (!bindingPath || !inputPath || (mode === 'verify' ? !trustPath : mode !== 'test' || trustPath)) {
    throw new Error(
      'Usage: node verify-wasm-windows.cjs verify <binding.js> <bundle.json> <trusted-keys.json>\n' +
      '       node verify-wasm-windows.cjs test <binding.js> <vectors.json>'
    );
  }
  const { verify_afir_bundle: verify } = require(resolve(bindingPath));
  const input = readFileSync(inputPath, 'utf8');

  if (mode === 'verify') {
    const report = verify(input, readFileSync(trustPath, 'utf8'));
    console.log(report);
    return;
  }

  const vectors = JSON.parse(input);
  const trust = JSON.stringify(vectors.trust);
  let accepted = 0;
  let rejected = 0;
  for (const { id, bundle, expected } of vectors.cases) {
    const run = () => verify(JSON.stringify(bundle), trust);
    if (expected === 'VALID') {
      const report = JSON.parse(run());
      assert.equal(report.mmr_size, bundle.receipts.length, id);
      assert.equal(report.receipts.length, bundle.receipts.length, id);
      assert.equal(report.checkpoints, bundle.checkpoints.length, id);
      assert.equal(report.attestation, 'NOT_VERIFIED', id);
      accepted++;
    } else {
      assert.throws(run, (error) => error === expected, id);
      rejected++;
    }
  }
  assert.ok(accepted > 0 && rejected > 0);
  console.log(JSON.stringify({ status: 'PASS', accepted, rejected, platform: process.platform }));
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exitCode = 1;
}
