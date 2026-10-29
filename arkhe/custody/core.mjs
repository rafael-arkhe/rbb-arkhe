// Experimental, synthetic-data pilot. Keys in fixtures are PUBLIC TEST KEYS.
import { createPrivateKey, createPublicKey, sign, verify } from "node:crypto";
import { keccak256, toUtf8Bytes } from "ethers";

export const FORMAT = "arkhe-custody/1";
export const LIMITS = Object.freeze({
  bytes: 2 * 1024 ** 3,
  file: 512 * 1024 ** 2,
  records: 1_000_000,
  depth: 16,
  string: 4096,
});
export function fail(code) {
  throw new Error(code);
}
export function canonical(value, depth = 0) {
  if (depth > LIMITS.depth) fail("RESOURCE_LIMIT");
  if (value === null || typeof value === "boolean")
    return JSON.stringify(value);
  if (typeof value === "string") {
    if (value.length > LIMITS.string || !/^[\x20-\x7e]*$/.test(value))
      fail("INVALID_STRING");
    return JSON.stringify(value);
  }
  if (Array.isArray(value))
    return "[" + value.map((v) => canonical(v, depth + 1)).join(",") + "]";
  if (value && Object.getPrototypeOf(value) === Object.prototype) {
    return (
      "{" +
      Object.keys(value)
        .sort()
        .map(
          (k) => canonical(k, depth + 1) + ":" + canonical(value[k], depth + 1),
        )
        .join(",") +
      "}"
    );
  }
  fail("INVALID_CANONICAL_VALUE");
}
export const digest = (value) => keccak256(toUtf8Bytes(canonical(value)));
export function parseCanonical(bytes) {
  if (Buffer.byteLength(bytes) > LIMITS.bytes) fail("RESOURCE_LIMIT");
  let value;
  try {
    value = JSON.parse(bytes.toString());
  } catch {
    fail("INVALID_JSON");
  }
  if (canonical(value) !== bytes.toString()) fail("NON_CANONICAL");
  return value;
}
function shape(value, keys) {
  if (
    !value ||
    Array.isArray(value) ||
    typeof value !== "object" ||
    Object.keys(value).sort().join(",") !== keys.split(" ").sort().join(",")
  )
    fail("SCHEMA");
}
function text(v) {
  if (typeof v !== "string" || !v.length) fail("SCHEMA");
  canonical(v);
}
function integer(v, signed = false) {
  if (
    typeof v !== "string" ||
    !(signed ? /^(0|-?[1-9][0-9]*)$/ : /^(0|[1-9][0-9]*)$/).test(v) ||
    v.length > 78
  )
    fail("INTEGER");
  return BigInt(v);
}
function list(v) {
  if (!Array.isArray(v) || v.length > LIMITS.records) fail("RESOURCE_LIMIT");
}
const seedKey = (seed) =>
  createPrivateKey({
    key: Buffer.concat([
      Buffer.from("302e020100300506032b657004220420", "hex"),
      Buffer.alloc(32, seed),
    ]),
    format: "der",
    type: "pkcs8",
  });
export function publicKey(seed) {
  return createPublicKey(seedKey(seed))
    .export({ format: "der", type: "spki" })
    .toString("hex");
}
export function signDocument(body, seed, domain = "ARKHE:CUSTODY:SOURCE:v1") {
  return {
    body,
    signature: sign(
      null,
      Buffer.from(domain + "\0" + canonical(body)),
      seedKey(seed),
    ).toString("hex"),
  };
}
export function verifyDocument(
  document,
  key,
  domain = "ARKHE:CUSTODY:SOURCE:v1",
) {
  try {
    shape(document, "body signature");
    if (!/^[0-9a-f]{128}$/.test(document.signature) || !/^[0-9a-f]+$/.test(key))
      return false;
    return verify(
      null,
      Buffer.from(domain + "\0" + canonical(document.body)),
      createPublicKey({
        key: Buffer.from(key, "hex"),
        format: "der",
        type: "spki",
      }),
      Buffer.from(document.signature, "hex"),
    );
  } catch {
    return false;
  }
}
export function validateInput(input) {
  shape(input, "format scope sources policy");
  if (input.format !== FORMAT) fail("VERSION");
  shape(input.scope, "network genesis asset decimals start end accounts");
  const scope = input.scope;
  for (const k of ["network", "genesis", "asset"]) text(scope[k]);
  integer(scope.decimals);
  integer(scope.start);
  integer(scope.end);
  if (integer(scope.start) >= integer(scope.end)) fail("PERIOD");
  list(scope.accounts);
  const accounts = new Set();
  for (const account of scope.accounts) {
    shape(account, "id classification");
    text(account.id);
    if (
      !["own", "clients"].includes(account.classification) ||
      accounts.has(account.id)
    )
      fail("ACCOUNT");
    accounts.add(account.id);
  }
  shape(input.policy, "id valid_from valid_until max_amount approvals");
  const p = input.policy;
  text(p.id);
  integer(p.valid_from);
  integer(p.valid_until);
  integer(p.max_amount);
  if (integer(p.valid_from) >= integer(p.valid_until)) fail("PERIOD");
  list(p.approvals);
  const approvals = new Set();
  for (const a of p.approvals) {
    shape(
      a,
      "id movement approver issued_at valid_from valid_until max_amount destination asset policy_id",
    );
    for (const k of [
      "id",
      "movement",
      "approver",
      "destination",
      "asset",
      "policy_id",
    ])
      text(a[k]);
    for (const k of ["issued_at", "valid_from", "valid_until", "max_amount"])
      integer(a[k]);
    if (approvals.has(a.id) || integer(a.valid_from) >= integer(a.valid_until))
      fail("APPROVAL");
    approvals.add(a.id);
  }
  shape(input.sources, "chain custody ledger");
  for (const [role, source] of Object.entries(input.sources)) {
    shape(
      source,
      "id owner origin dependencies available gaps start end accounts opening closing records blocks",
    );
    for (const k of ["id", "owner", "origin"]) text(source[k]);
    if (source.id !== role || typeof source.available !== "boolean")
      fail("SOURCE");
    integer(source.start);
    integer(source.end);
    for (const k of ["dependencies", "gaps", "accounts", "records", "blocks"])
      list(source[k]);
    for (const k of ["dependencies", "gaps", "accounts"])
      source[k].forEach(text);
    if (new Set(source.accounts).size !== source.accounts.length)
      fail("ACCOUNT");
    for (const balances of [source.opening, source.closing]) {
      shape(balances, [...accounts].join(" "));
      Object.values(balances).forEach((v) => integer(v));
    }
    for (const r of source.records) {
      shape(
        r,
        "id movement account delta time block_number block_hash tx log_index classification authorization",
      );
      for (const k of [
        "id",
        "movement",
        "account",
        "block_hash",
        "tx",
        "classification",
        "authorization",
      ])
        text(r[k]);
      if (
        !accounts.has(r.account) ||
        !["own", "clients"].includes(r.classification)
      )
        fail("ACCOUNT");
      integer(r.delta, true);
      for (const k of ["time", "block_number", "log_index"]) integer(r[k]);
      if (
        integer(r.time) < integer(scope.start) ||
        integer(r.time) >= integer(scope.end)
      )
        fail("PERIOD");
    }
    for (const b of source.blocks) {
      shape(b, "number initial_hash final_hash head");
      integer(b.number);
      integer(b.head);
      text(b.initial_hash);
      text(b.final_hash);
    }
  }
  canonical(input);
}

export function reconcile(input) {
  validateInput(input);
  const exceptions = [];
  const add = (code, subject) => exceptions.push({ code, subject });
  const normalized = {};
  const coverage = {};
  for (const [role, s] of Object.entries(input.sources)) {
    let covered =
      s.available &&
      !s.gaps.length &&
      s.start === input.scope.start &&
      s.end === input.scope.end &&
      input.scope.accounts.every((a) => s.accounts.includes(a.id));
    if (!covered) add("SOURCE_COVERAGE_GAP", role);
    if (s.available)
      for (const b of s.blocks) {
        if (b.initial_hash !== b.final_hash) {
          add("REFERENCE_BLOCK_CONFLICT", role + ":" + b.number);
          covered = false;
        }
        if (integer(b.head) - integer(b.number) < 3n) {
          add("INSUFFICIENT_CONFIRMATIONS", role + ":" + b.number);
          covered = false;
        }
      }
    const byId = new Map(),
      byEvent = new Map(),
      blockMap = new Map();
    for (const b of s.blocks) {
      if (
        blockMap.has(b.number) &&
        canonical(blockMap.get(b.number)) !== canonical(b)
      ) {
        add("REFERENCE_BLOCK_CONFLICT", role + ":" + b.number);
        covered = false;
      } else blockMap.set(b.number, b);
    }
    for (const r of s.available ? s.records : []) {
      if (
        role === "chain" &&
        blockMap.get(r.block_number)?.initial_hash !== r.block_hash
      ) {
        add("BLOCK_REFERENCE_MISSING", role + ":" + r.id);
        covered = false;
      }
      if (byId.has(r.id))
        add(
          canonical(byId.get(r.id)) === canonical(r)
            ? "DUPLICATE_EVENT"
            : "IDENTIFIER_CONFLICT",
          role + ":" + r.id,
        );
      else {
        const eventKey = canonical({
          tx: r.tx,
          log_index: r.log_index,
          account: r.account,
        });
        if (byEvent.has(eventKey)) {
          const first = byEvent.get(eventKey),
            same = canonical({ ...first, id: r.id }) === canonical(r);
          add(
            same ? "DUPLICATE_EVENT" : "IDENTIFIER_CONFLICT",
            role + ":" + r.id,
          );
        } else {
          byId.set(r.id, r);
          byEvent.set(eventKey, r);
        }
      }
      const account = input.scope.accounts.find((a) => a.id === r.account);
      if (account.classification !== r.classification)
        add("CLASSIFICATION_CONFLICT", role + ":" + r.id);
    }
    normalized[role] = [...byId.values()].sort((a, b) =>
      a.id < b.id ? -1 : a.id > b.id ? 1 : 0,
    );
    coverage[role] = covered;
    if (covered)
      for (const a of input.scope.accounts) {
        const change = normalized[role]
          .filter((r) => r.account === a.id)
          .reduce((n, r) => n + integer(r.delta, true), 0n);
        if (integer(s.opening[a.id]) + change !== integer(s.closing[a.id]))
          add("BALANCE_MISMATCH", role + ":" + a.id);
      }
  }
  for (const role of ["custody", "ledger"]) {
    if (!coverage.chain || !coverage[role]) continue;
    const identity = (r) =>
      canonical({
        movement: r.movement,
        account: r.account,
        delta: r.delta,
        tx: r.tx,
        log_index: r.log_index,
        block_hash: r.block_hash,
        block_number: r.block_number,
        time: r.time,
      });
    const left = new Set(normalized.chain.map(identity)),
      right = new Set(normalized[role].map(identity));
    for (const r of normalized.chain)
      if (!right.has(identity(r)))
        add("MOVEMENT_MISSING_OR_DIFFERENT", role + ":" + r.id);
    for (const r of normalized[role])
      if (!left.has(identity(r))) add("MOVEMENT_UNOBSERVED", role + ":" + r.id);
    for (const a of input.scope.accounts)
      for (const cut of ["opening", "closing"])
        if (input.sources.chain[cut][a.id] !== input.sources[role][cut][a.id])
          add("CROSS_SOURCE_BALANCE_MISMATCH", role + ":" + cut + ":" + a.id);
  }
  const p = input.policy,
    approvals = new Map(p.approvals.map((a) => [a.id, a]));
  for (const r of normalized.chain) {
    const approval = approvals.get(r.authorization);
    const time = integer(r.time),
      amount = integer(r.delta, true);
    const magnitude = amount < 0n ? -amount : amount;
    if (
      !approval ||
      approval.movement !== r.movement ||
      approval.asset !== input.scope.asset ||
      approval.destination !== r.account ||
      approval.policy_id !== p.id ||
      time < integer(p.valid_from) ||
      time >= integer(p.valid_until) ||
      time < integer(approval.valid_from) ||
      time >= integer(approval.valid_until) ||
      integer(approval.issued_at) > time ||
      magnitude > integer(p.max_amount) ||
      magnitude > integer(approval.max_amount)
    )
      add("AUTHORIZATION_VIOLATION", r.id);
  }
  const dependencyRoles = new Map();
  for (const [role, s] of Object.entries(input.sources))
    for (const d of s.dependencies) {
      const roles = dependencyRoles.get(d) ?? new Set();
      roles.add(role);
      dependencyRoles.set(d, roles);
    }
  for (const [d, roles] of dependencyRoles)
    if (roles.size > 1) add("SHARED_DEPENDENCY", d);
  const unique = [
    ...new Map(exceptions.map((e) => [canonical(e), e])).values(),
  ].sort((a, b) =>
    canonical(a) < canonical(b) ? -1 : canonical(a) > canonical(b) ? 1 : 0,
  );
  const authorization = unique.some((e) => e.code === "AUTHORIZATION_VIOLATION")
    ? "violation"
    : coverage.chain
      ? "valid_in_scope"
      : "not_evaluable";
  const divergent = unique.some((e) =>
    [
      "IDENTIFIER_CONFLICT",
      "CLASSIFICATION_CONFLICT",
      "BALANCE_MISMATCH",
      "MOVEMENT_MISSING_OR_DIFFERENT",
      "MOVEMENT_UNOBSERVED",
      "CROSS_SOURCE_BALANCE_MISMATCH",
    ].includes(e.code),
  );
  return {
    format: FORMAT,
    coverage: Object.values(coverage).every(Boolean)
      ? "sufficient_for_declared_rules"
      : "insufficient",
    reconciliation: divergent
      ? "divergent"
      : Object.values(coverage).every(Boolean)
        ? "consistent_in_scope"
        : "not_evaluable",
    authorization,
    exceptions: unique,
    limits: [
      "Unknown wallets and jointly omitted operations cannot be detected.",
      "Synthetic sources do not demonstrate operational independence.",
      "This report is not a solvency or regulatory certification.",
    ],
  };
}

export const RULESET = {
  id: "arkhe-custody-rules/1",
  semantics:
    "quantities;scope;three-source;authorization;half-open;depth3;duplicates;classification;shared-dependencies",
};
export const RULESET_DIGEST = digest(RULESET);
export function testTrust() {
  return {
    format: FORMAT,
    rulesets: [RULESET_DIGEST],
    sources: {
      chain: publicKey(11),
      custody: publicKey(12),
      ledger: publicKey(13),
    },
    anchor: publicKey(14),
    test_keys_public: true,
  };
}
export function makePackage(input, report = reconcile(input)) {
  const files = [
    { path: "input.json", digest: digest(input) },
    { path: "report.json", digest: digest(report) },
    { path: "rules.json", digest: RULESET_DIGEST },
  ];
  const sources = Object.fromEntries(
    Object.entries(input.sources).map(([id, body]) => [
      id,
      signDocument(
        { scope_digest: digest(input.scope), source: body },
        { chain: 11, custody: 12, ledger: 13 }[id],
      ),
    ]),
  );
  return {
    format: FORMAT,
    input,
    report,
    rules: structuredClone(RULESET),
    sources,
    manifest: {
      format: FORMAT,
      files,
      policy_digest: digest(input.policy),
      scope_digest: digest(input.scope),
    },
    anchor: null,
  };
}
export function replay(pkg, trust) {
  shape(trust, "format rulesets sources anchor test_keys_public");
  shape(trust.sources, "chain custody ledger");
  if (trust.test_keys_public !== true) fail("TEST_ONLY_TRUST_REQUIRED");
  shape(pkg, "format input report rules sources manifest anchor");
  if (pkg.format !== FORMAT || trust.format !== FORMAT) fail("VERSION");
  if (
    !trust.rulesets.includes(digest(pkg.rules)) ||
    digest(pkg.rules) !== RULESET_DIGEST
  )
    fail("RULESET_NOT_ALLOWED");
  shape(pkg.manifest, "format files policy_digest scope_digest");
  if (pkg.manifest.format !== FORMAT) fail("VERSION");
  const expected = [
    { path: "input.json", digest: digest(pkg.input) },
    { path: "report.json", digest: digest(pkg.report) },
    { path: "rules.json", digest: RULESET_DIGEST },
  ];
  if (canonical(pkg.manifest.files) !== canonical(expected)) {
    if (
      pkg.manifest.files.find((f) => f.path === "report.json")?.digest !==
      digest(pkg.report)
    )
      fail("REPORT_DIGEST_MISMATCH");
    fail("MANIFEST_DIGEST_MISMATCH");
  }
  if (
    pkg.manifest.policy_digest !== digest(pkg.input.policy) ||
    pkg.manifest.scope_digest !== digest(pkg.input.scope)
  )
    fail("MANIFEST_DIGEST_MISMATCH");
  shape(pkg.sources, "chain custody ledger");
  for (const [id, source] of Object.entries(pkg.sources)) {
    if (
      !verifyDocument(source, trust.sources[id]) ||
      canonical(source.body) !==
        canonical({
          scope_digest: digest(pkg.input.scope),
          source: pkg.input.sources[id],
        })
    )
      fail("SOURCE_AUTHENTICITY");
  }
  const recalculated = reconcile(pkg.input);
  if (canonical(recalculated) !== canonical(pkg.report))
    fail("REPORT_REPLAY_MISMATCH");
  let signature = false;
  if (pkg.anchor !== null) {
    signature = verifyDocument(
      pkg.anchor,
      trust.anchor,
      "ARKHE:CUSTODY:ANCHOR:v1",
    );
    if (!signature || pkg.anchor.body.manifest_digest !== digest(pkg.manifest))
      fail("ANCHOR_ATTESTATION");
  }
  return {
    report: recalculated,
    integrity: "valid",
    anchor: {
      attestation_signature_valid: signature,
      chain_observation_independently_verified: false,
      test_keys_public: trust.test_keys_public,
      mode: pkg.anchor ? "LOCAL_OPERATOR_ATTESTED" : "NOT_PROVIDED",
    },
  };
}
