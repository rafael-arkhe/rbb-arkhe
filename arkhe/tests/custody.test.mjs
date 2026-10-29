import test from "node:test";
import assert from "node:assert/strict";
import {
  canonical,
  parseCanonical,
  reconcile,
  makePackage,
  replay,
  testTrust,
  digest,
  signDocument,
} from "../custody/core.mjs";
import { scenario } from "../custody/scenarios.mjs";
const codes = (input) => reconcile(input).exceptions.map((e) => e.code);
test("custody: deterministic package and offline replay", () => {
  const p = makePackage(scenario());
  assert.equal(canonical(p), canonical(makePackage(scenario())));
  assert.deepEqual(
    replay(parseCanonical(Buffer.from(canonical(p))), testTrust()).report,
    p.report,
  );
});
test("custody: missing transfer is detected", () => {
  const s = scenario();
  s.sources.custody.records.pop();
  assert.ok(codes(s).includes("MOVEMENT_MISSING_OR_DIFFERENT"));
});
test("custody: duplicate is identified without double counting", () => {
  const s = scenario();
  s.sources.chain.records.push(structuredClone(s.sources.chain.records[0]));
  const c = codes(s);
  assert.ok(c.includes("DUPLICATE_EVENT"));
  assert.ok(!c.includes("BALANCE_MISMATCH"));
});
test("custody: duplicate chain identity with a different local ID is not counted twice", () => {
  const s = scenario();
  s.sources.chain.records.push({
    ...s.sources.chain.records[0],
    id: "different-id",
  });
  const c = codes(s);
  assert.ok(c.includes("DUPLICATE_EVENT"));
  assert.ok(!c.includes("BALANCE_MISMATCH"));
});
test("custody: conflicting identifier and classification", () => {
  const s = scenario();
  s.sources.custody.records.push({
    ...s.sources.custody.records[0],
    delta: "2",
    classification: "own",
  });
  const c = codes(s);
  assert.ok(c.includes("IDENTIFIER_CONFLICT"));
  assert.ok(c.includes("CLASSIFICATION_CONFLICT"));
});
test("custody: authorization amount, binding and half-open time", () => {
  for (const change of [
    (a) => (a.max_amount = "0"),
    (a) => (a.movement = "wrong"),
    (a) => (a.valid_until = String(1803772810)),
    (a) => (a.issued_at = String(1803772811)),
  ]) {
    const s = scenario();
    change(s.policy.approvals[0]);
    assert.ok(codes(s).includes("AUTHORIZATION_VIOLATION"));
  }
  const s = scenario();
  s.policy.approvals[0].valid_from = String(1803772810);
  assert.ok(!codes(s).includes("AUTHORIZATION_VIOLATION"));
  s.policy.approvals[0].valid_until = String(1803772811);
  assert.ok(!codes(s).includes("AUTHORIZATION_VIOLATION"));
});
test("custody: coverage gap does not erase independently observed divergence", () => {
  const s = scenario();
  s.sources.ledger.available = false;
  s.sources.custody.records.pop();
  const r = reconcile(s);
  assert.equal(r.coverage, "insufficient");
  assert.equal(r.reconciliation, "divergent");
});
test("custody: insufficient confirmations and replacement are inconclusive", () => {
  for (const mutate of [
    (b) => (b.final_hash = "replacement"),
    (b) => (b.head = "2"),
  ]) {
    const s = scenario();
    mutate(s.sources.chain.blocks[0]);
    assert.equal(reconcile(s).reconciliation, "not_evaluable");
  }
});
test("custody: shared dependencies are explicit", () => {
  const s = scenario();
  s.sources.chain.dependencies.push("shared");
  s.sources.custody.dependencies.push("shared");
  assert.ok(codes(s).includes("SHARED_DEPENDENCY"));
});
test("custody: unknown wallet is a declared limit, not discovered", () => {
  const r = reconcile(scenario());
  assert.equal(r.reconciliation, "consistent_in_scope");
  assert.ok(r.limits[0].includes("cannot be detected"));
});
test("custody: report mutation, authenticated false report and rules swap differ", () => {
  let p = makePackage(scenario());
  p.report.coverage = "insufficient";
  assert.throws(() => replay(p, testTrust()), /REPORT_DIGEST_MISMATCH/);
  p = makePackage(scenario(), {
    ...reconcile(scenario()),
    coverage: "insufficient",
  });
  assert.throws(() => replay(p, testTrust()), /REPORT_REPLAY_MISMATCH/);
  p = makePackage(scenario());
  p.rules.id = "other";
  assert.throws(() => replay(p, testTrust()), /RULESET_NOT_ALLOWED/);
});
test("custody: external trust, source replacement and invalid signatures rejected", () => {
  const p = makePackage(scenario());
  const trust = testTrust();
  trust.sources.chain = trust.sources.custody;
  assert.throws(() => replay(p, trust), /SOURCE_AUTHENTICITY/);
  p.sources.chain.signature = "00".repeat(64);
  assert.throws(() => replay(p, testTrust()), /SOURCE_AUTHENTICITY/);
});
test("custody: local attestation is explicitly not independent chain verification", () => {
  const p = makePackage(scenario());
  p.anchor = signDocument(
    { manifest_digest: digest(p.manifest) },
    14,
    "ARKHE:CUSTODY:ANCHOR:v1",
  );
  const r = replay(p, testTrust());
  assert.equal(r.anchor.attestation_signature_valid, true);
  assert.equal(r.anchor.chain_observation_independently_verified, false);
  assert.equal(r.anchor.test_keys_public, true);
});
test("custody: strict canonical parser and schemas reject ambiguity", () => {
  for (const s of [
    '{"a":"1","a":"1"}',
    '{"a":1}',
    ' {"a":"1"}',
    '{"b":"1","a":"1"}',
  ])
    assert.throws(() => parseCanonical(Buffer.from(s)));
  const s = scenario();
  s.scope.extra = "x";
  assert.throws(() => reconcile(s), /SCHEMA/);
  s.scope.extra = undefined;
  assert.throws(() => canonical(s));
});
