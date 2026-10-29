import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";
import { digest, fail } from "./core.mjs";
import { AbiCoder, keccak256, concat, toUtf8Bytes } from "ethers";
import { readFileSync } from "node:fs";
const root = fileURLToPath(new URL("../", import.meta.url));
function call(name, request) {
  const binary = resolve(
    root,
    "protocol/target/debug/examples/" +
      name +
      (process.platform === "win32" ? ".exe" : ""),
  );
  return JSON.parse(
    execFileSync(binary, [], {
      input: JSON.stringify(request),
      maxBuffer: 1024 * 1024,
      stdio: ["pipe", "pipe", "pipe"],
    }).toString(),
  );
}
export function mmrRequest(pkg) {
  return {
    records: pkg.manifest.files.map((entry) => digest(entry).slice(2)),
    manifest_digest: digest(pkg.manifest).slice(2),
    envelope: null,
  };
}
export function frontier(pkg) {
  return call("custody_bridge", mmrRequest(pkg));
}
export function signedCheckpoint(header) {
  const signed = call("rbb_sign_fixture", header);
  // Strings for all domain integers; long signature split into bounded chunks.
  const h = Object.fromEntries(
    Object.entries(signed.envelope.header).map(([k, v]) => [
      k,
      Array.isArray(v)
        ? "0x" + Buffer.from(v).toString("hex")
        : typeof v === "number"
          ? String(v)
          : v,
    ]),
  );
  return {
    header: h,
    ed: Buffer.from(signed.envelope.ed25519_signature).toString("hex"),
    ml: Buffer.from(signed.envelope.mldsa_signature)
      .toString("hex")
      .match(/.{1,2048}/g),
  };
}
export function rawEnvelope(checkpoint) {
  const header = Object.fromEntries(
    Object.entries(checkpoint.header).map(([k, v]) => [
      k,
      typeof v === "string" && v.startsWith("0x")
        ? [...Buffer.from(v.slice(2), "hex")]
        : typeof v === "string"
          ? Number(v)
          : v,
    ]),
  );
  for (const v of Object.values(header))
    if (typeof v === "number" && !Number.isSafeInteger(v)) fail("INTEGER");
  return {
    header,
    ed25519_signature: [...Buffer.from(checkpoint.ed, "hex")],
    mldsa_signature: [...Buffer.from(checkpoint.ml.join(""), "hex")],
  };
}
export function verifyCheckpoint(pkg) {
  const checkpoint = pkg.anchor?.body.checkpoint;
  if (!checkpoint) fail("CHECKPOINT_MISSING");
  const h = checkpoint.header;
  if (
    h.source_kind !== "4" ||
    h.policy_digest !== pkg.manifest.policy_digest ||
    h.source_manifest_digest !== digest(pkg.input.sources) ||
    h.source_manifest_present !== true ||
    h.anchor_genesis_hash !== pkg.input.scope.genesis ||
    h.anchor_chain_id !== "0x" + BigInt(1337).toString(16).padStart(64, "0")
  )
    fail("CHECKPOINT_BINDING");
  const vector = JSON.parse(
    readFileSync(resolve(root, "fixtures/protocol-v3.json"), "utf8"),
  );
  if (
    h.org_id !== keccak256(toUtf8Bytes("ARKHE:CUSTODY:PILOT:ORG:1")) ||
    h.stream_id !== keccak256(toUtf8Bytes("ARKHE:CUSTODY:PILOT:STREAM:1")) ||
    h.power !== String(vector.header.power) ||
    h.source_network_id !==
      keccak256(
        toUtf8Bytes(
          "ARKHE:CUSTODY:PILOT:EVM:1337:" + pkg.input.scope.genesis.slice(2),
        ),
      )
  )
    fail("CHECKPOINT_BINDING");
  const coder = AbiCoder.defaultAbiCoder(),
    values = Object.keys(vector.header).map((k) => h[k]),
    tuple = "tuple(" + vector.abi_types.join(",") + ")";
  const id = keccak256(
      concat([
        keccak256(toUtf8Bytes("AKG:CHECKPOINT:HEADER:v3\0")),
        coder.encode([tuple], [values]),
      ]),
    ),
    commitment = keccak256(
      coder.encode(
        [tuple, "bytes", "bytes"],
        [values, "0x" + checkpoint.ed, "0x" + checkpoint.ml.join("")],
      ),
    );
  const obs = pkg.anchor.body.observation;
  if (
    !obs ||
    obs.mode !== "LOCAL_OPERATOR_ATTESTED" ||
    obs.genesis !== h.anchor_genesis_hash ||
    obs.chain_id !== "1337" ||
    obs.contract !== h.anchor_address ||
    obs.checkpoint_id !== id ||
    obs.envelope_commitment !== commitment ||
    obs.root !== h.mmr_root ||
    obs.count !== h.total_leaf_count ||
    obs.receipt_status !== "1"
  )
    fail("ANCHOR_OBSERVATION_BINDING");
  call("custody_bridge", {
    ...mmrRequest(pkg),
    envelope: rawEnvelope(checkpoint),
  });
  return {
    hybrid_signatures_valid: true,
    mmr_full_frontier_recomputed: true,
    proof_mode: "FULL_SINGLE_BATCH_RECOMPUTATION",
  };
}
