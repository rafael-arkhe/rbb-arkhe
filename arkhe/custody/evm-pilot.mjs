import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";
import solc from "solc";
import ganache from "ganache";
import {
  BrowserProvider,
  ContractFactory,
  keccak256,
  toUtf8Bytes,
  hexlify,
} from "ethers";
import assert from "node:assert/strict";
import { scenario } from "./scenarios.mjs";
import {
  canonical,
  digest,
  makePackage,
  replay,
  signDocument,
  testTrust,
  reconcile,
} from "./core.mjs";
import { frontier, signedCheckpoint, verifyCheckpoint } from "./checkpoint.mjs";

const root = fileURLToPath(new URL("../", import.meta.url));
const get = (name) => readFileSync(resolve(root, "contracts", name), "utf8");
const token =
  "pragma solidity 0.8.30; contract TestToken { mapping(address=>uint256) public balanceOf; address immutable owner=msg.sender; event Transfer(address indexed from,address indexed to,uint256 value); function mint(address to,uint256 value) external { require(msg.sender==owner); balanceOf[to]+=value; emit Transfer(address(0),to,value); } }";
export async function executePilot(outputDirectory) {
  const sources = {
    "ArkheCheckpointAnchor.sol": { content: get("ArkheCheckpointAnchor.sol") },
    "RegistryFixture.sol": { content: get("fixtures/RegistryFixture.sol") },
    "TestToken.sol": { content: token },
  };
  const compiled = JSON.parse(
    solc.compile(
      JSON.stringify({
        language: "Solidity",
        sources,
        settings: {
          optimizer: { enabled: true, runs: 200 },
          viaIR: true,
          evmVersion: "shanghai",
          outputSelection: { "*": { "*": ["abi", "evm.bytecode.object"] } },
        },
      }),
    ),
  );
  assert.deepEqual(
    (compiled.errors ?? []).filter((e) => e.severity === "error"),
    [],
  );
  const local = ganache.provider({
    logging: { quiet: true },
    chain: {
      chainId: 1337,
      hardfork: "shanghai",
      time: new Date(1803772800 * 1000),
    },
    miner: { timestampIncrement: 1 },
    wallet: { deterministic: true, totalAccounts: 3 },
  });
  try {
    const provider = new BrowserProvider(local);
    provider.pollingInterval = 10;
    const admin = await provider.getSigner(0),
      operator = await provider.getSigner(1),
      client = await provider.getSigner(2);
    const deploy = async (file, name, args = []) => {
      const a = compiled.contracts[file][name];
      const c = await new ContractFactory(
        a.abi,
        a.evm.bytecode.object,
        admin,
      ).deploy(...args);
      await c.waitForDeployment();
      return c;
    };
    const block = async (n) =>
      provider.send("eth_getBlockByNumber", [
        typeof n === "string" ? n : "0x" + n.toString(16),
        false,
      ]);
    const genesis = (await block(0)).hash;
    const registry = await deploy("RegistryFixture.sol", "RegistryFixture");
    const anchor = await deploy(
      "ArkheCheckpointAnchor.sol",
      "ArkheCheckpointAnchor",
      [await registry.getAddress(), genesis],
    );
    const asset = await deploy("TestToken.sol", "TestToken");
    const input = scenario(100);
    input.scope.genesis = genesis;
    input.scope.asset = (await asset.getAddress()).toLowerCase();
    input.scope.end = String(1803782800);
    const records = [];
    for (let i = 0; i < 100; i++) {
      const receipt = await (
        await asset.mint(await client.getAddress(), 1)
      ).wait();
      const b = await block(receipt.blockNumber);
      const log = receipt.logs[0];
      assert.equal(asset.interface.parseLog(log).args.value, 1n);
      records.push({
        ...input.sources.chain.records[i],
        time: BigInt(b.timestamp).toString(),
        block_number: BigInt(b.number).toString(),
        block_hash: b.hash,
        tx: receipt.hash,
        log_index: String(log.index),
      });
    }
    for (let i = 0; i < 3; i++) await provider.send("evm_mine", []);
    const head = BigInt((await block("latest")).number);
    for (const [id, s] of Object.entries(input.sources)) {
      s.end = input.scope.end;
      s.records = structuredClone(records);
      s.origin = id === "chain" ? "local-evm-rpc" : "synthetic-" + id;
      s.blocks = records.map((r) => ({
        number: r.block_number,
        initial_hash: r.block_hash,
        final_hash: r.block_hash,
        head: head.toString(),
      }));
    }
    input.policy.valid_until = input.scope.end;
    for (const a of input.policy.approvals) {
      a.asset = input.scope.asset;
      a.valid_until = input.scope.end;
    }
    assert.equal(await asset.balanceOf(await client.getAddress()), 100n);
    const pkg = makePackage(input);
    assert.equal(pkg.report.reconciliation, "consistent_in_scope");
    const f = frontier(pkg);
    const fixture = JSON.parse(
      readFileSync(resolve(root, "fixtures/protocol-v3.json"), "utf8"),
    );
    const h = { ...fixture.header };
    const bytes = (x) => [...Buffer.from(x.replace(/^0x/, ""), "hex")];
    h.anchor_genesis_hash = bytes(genesis);
    h.registry_address = bytes(await registry.getAddress());
    h.anchor_address = bytes(await anchor.getAddress());
    h.org_id = bytes(keccak256(toUtf8Bytes("ARKHE:CUSTODY:PILOT:ORG:1")));
    h.stream_id = bytes(keccak256(toUtf8Bytes("ARKHE:CUSTODY:PILOT:STREAM:1")));
    h.source_kind = 4;
    h.source_network_id = bytes(
      keccak256(
        toUtf8Bytes("ARKHE:CUSTODY:PILOT:EVM:1337:" + genesis.slice(2)),
      ),
    );
    h.source_manifest_present = true;
    h.source_manifest_digest = bytes(digest(input.sources));
    h.policy_digest = bytes(pkg.manifest.policy_digest);
    h.mmr_root = bytes(f.root);
    h.mmr_profile_digest = bytes(f.profile);
    h.batch_manifest_digest = bytes(digest(pkg.manifest));
    h.sequence = 1;
    h.prev_present = false;
    h.prev_checkpoint_id = Array(32).fill(0);
    h.first_leaf_index = 0;
    h.batch_leaf_count = 3;
    h.total_leaf_count = 3;
    h.declared_signed_at = Number(BigInt((await block("latest")).timestamp));
    const checkpoint = signedCheckpoint(h);
    const ed = "0x" + checkpoint.ed,
      ml = "0x" + checkpoint.ml.join("");
    await (
      await registry.setOrg(
        hexlify(Uint8Array.from(h.org_id)),
        await operator.getAddress(),
        h.power,
        true,
      )
    ).wait();
    await (
      await registry.setKeys(
        hexlify(Uint8Array.from(h.org_id)),
        1,
        checkpoint.header.ed25519_key_id,
        checkpoint.header.mldsa_key_id,
        1803772700,
        1803782800,
        true,
      )
    ).wait();
    await (
      await anchor.configureStream(
        checkpoint.header.org_id,
        checkpoint.header.stream_id,
        checkpoint.header.mmr_profile_digest,
        checkpoint.header.source_network_id,
        checkpoint.header.policy_digest,
        4,
      )
    ).wait();
    const header = Object.fromEntries(
      Object.entries(checkpoint.header).map(([k, v]) => [
        k,
        k === "anchor_chain_id" ? BigInt(v) : v,
      ]),
    );
    const receipt = await (
      await anchor.connect(operator).record(header, ed, ml)
    ).wait();
    assert.equal(receipt.status, 1);
    const event = anchor.interface.parseLog(receipt.logs[0]);
    assert.equal(
      receipt.logs[0].address.toLowerCase(),
      (await anchor.getAddress()).toLowerCase(),
    );
    const id = await anchor.checkpointId(header),
      commitment = await anchor.envelopeCommitment(header, ed, ml);
    assert.equal(event.args.checkpointId, id);
    assert.equal(event.args.commitment, commitment);
    const record = await anchor.records(
      await anchor.streamKey(header.org_id, header.stream_id),
      1,
    );
    assert.equal(record.exists, true);
    assert.equal(record.id, id);
    assert.equal(record.commitment, commitment);
    assert.equal(record.root, header.mmr_root);
    assert.equal(record.count, 3n);
    assert.equal(record.blockNumber, BigInt(receipt.blockNumber));
    for (let i = 0; i < 3; i++) await provider.send("evm_mine", []);
    const observed = await block(receipt.blockNumber);
    for (const r of records)
      assert.equal((await block(Number(r.block_number))).hash, r.block_hash);
    pkg.anchor = signDocument(
      {
        manifest_digest: digest(pkg.manifest),
        checkpoint,
        observation: {
          mode: "LOCAL_OPERATOR_ATTESTED",
          genesis,
          chain_id: "1337",
          contract: (await anchor.getAddress()).toLowerCase(),
          runtime_code_digest: keccak256(
            await provider.send("eth_getCode", [
              await anchor.getAddress(),
              "latest",
            ]),
          ),
          transaction: receipt.hash,
          block_number: String(receipt.blockNumber),
          block_hash: observed.hash,
          checkpoint_id: id,
          envelope_commitment: commitment,
          root: record.root,
          count: record.count.toString(),
          receipt_status: "1",
        },
      },
      14,
      "ARKHE:CUSTODY:ANCHOR:v1",
    );
    const result = {
      ...replay(pkg, testTrust()),
      checkpoint: verifyCheckpoint(pkg),
    };
    // Actual alternative branch, exported as a separate negative scenario.
    const snapshot = await provider.send("evm_snapshot", []);
    const orphanReceipt = await (
      await asset.mint(await client.getAddress(), 1)
    ).wait();
    const orphan = await block(orphanReceipt.blockNumber);
    assert.equal(await provider.send("evm_revert", [snapshot]), true);
    const replacementReceipt = await (
      await asset.mint(await client.getAddress(), 2)
    ).wait();
    const replacement = await block(replacementReceipt.blockNumber);
    assert.equal(orphan.number, replacement.number);
    assert.notEqual(orphan.hash, replacement.hash);
    const conflict = structuredClone(input);
    conflict.sources.chain.blocks.push({
      number: BigInt(orphan.number).toString(),
      initial_hash: orphan.hash,
      final_hash: replacement.hash,
      head: (BigInt(orphan.number) + 3n).toString(),
    });
    const reorgReport = reconcile(conflict);
    assert.equal(reorgReport.coverage, "insufficient");
    assert.ok(
      reorgReport.exceptions.some((e) => e.code === "REFERENCE_BLOCK_CONFLICT"),
    );
    const metadata = {
      format: "arkhe-custody-execution/1",
      solc: solc.version(),
      chain_id: "1337",
      transfers: "100",
      test_keys_public: true,
      source_digests: Object.fromEntries(
        Object.entries(sources).map(([name, s]) => [
          name,
          keccak256(toUtf8Bytes(s.content)),
        ]),
      ),
      result,
      reorg: reorgReport,
    };
    if (outputDirectory) {
      mkdirSync(outputDirectory, { recursive: true });
      for (const [name, value] of Object.entries({
        "package.json": pkg,
        "trust.json": testTrust(),
        "result.json": result,
        "execution.json": metadata,
      }))
        writeFileSync(resolve(outputDirectory, name), canonical(value));
    }
    return { pkg, result, metadata };
  } finally {
    await local.disconnect();
  }
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  executePilot(
    resolve(process.argv[2] ?? resolve(root, "test-results/custody")),
  )
    .then((r) =>
      process.stdout.write(
        JSON.stringify({
          transfers: r.metadata.transfers,
          result: r.result.report.reconciliation,
          anchor: r.result.anchor,
        }) + "\n",
      ),
    )
    .catch((e) => {
      console.error(e);
      process.exitCode = 1;
    });
}
