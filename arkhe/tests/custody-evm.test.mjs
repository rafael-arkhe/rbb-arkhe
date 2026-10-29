import test from "node:test";
import assert from "node:assert/strict";
import { executePilot } from "../custody/evm-pilot.mjs";
import { verifyCheckpoint } from "../custody/checkpoint.mjs";
test("custody: 100 EVM transfers, hybrid checkpoint, storage, offline recomputation and actual replacement branch", async () => {
  const { pkg, result } = await executePilot();
  assert.equal(result.report.reconciliation, "consistent_in_scope");
  assert.equal(result.checkpoint.hybrid_signatures_valid, true);
  assert.equal(result.anchor.chain_observation_independently_verified, false);
  const changed = structuredClone(pkg);
  changed.anchor.body.checkpoint.header.power = "2";
  assert.throws(() => verifyCheckpoint(changed));
  changed.anchor.body.checkpoint.header.mmr_root = "0x" + "00".repeat(32);
  assert.throws(() => verifyCheckpoint(changed));
});
