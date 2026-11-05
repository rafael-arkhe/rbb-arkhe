import { createHash } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { parseArgs } from "node:util";

const owner = "0xb18322Ea7c9AB4d47e29534A2E352e8A9e3beeFD";
const tx = "0xa48d2d40a951111762ff27ff6a8b06b2d55f9a29a9e01291cbf4b6bf380c59dd";
const token = "0xdac17f958d2ee523a2206206994597c13d831ec7";
const transfer = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
const support = "https://cs.tether.to/TR";
const topic = (address) => "0x" + address.toLowerCase().slice(2).padStart(64, "0");

const { values } = parseArgs({
  options: {
    output: { type: "string" },
    rpc: { type: "string", default: "https://ethereum-rpc.publicnode.com" },
    "qvac-config": { type: "string" },
  },
});
if (!values.output) throw new Error("usage: node src/tether-recovery.mjs --output <new-directory> [--qvac-config config.json]");
const endpoint = new URL(values.rpc);
if (endpoint.protocol !== "https:" || endpoint.username || endpoint.password) {
  throw new Error("use an HTTPS Ethereum RPC URL without credentials in userinfo");
}

async function rpc(method, params = []) {
  const response = await fetch(endpoint, {
    method: "POST",
    redirect: "error",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", id: 1, method, params }),
    signal: AbortSignal.timeout(15000),
  });
  if (!response.ok) throw new Error(`RPC ${method} returned HTTP ${response.status}`);
  const reply = await response.json();
  if (reply.id !== 1 || reply.error || reply.result == null) {
    throw new Error(`RPC returned no usable result for ${method}`);
  }
  return reply.result;
}

const [chainId, receipt, finalized] = await Promise.all([
  rpc("eth_chainId"),
  rpc("eth_getTransactionReceipt", [tx]),
  rpc("eth_getBlockByNumber", ["finalized", false]),
]);
if (chainId !== "0x1" || receipt.transactionHash !== tx || receipt.status !== "0x1"
    || receipt.from?.toLowerCase() !== owner.toLowerCase()
    || BigInt(receipt.blockNumber) > BigInt(finalized.number)) {
  throw new Error("RPC data does not confirm the requested finalized Ethereum transaction");
}
const block = await rpc("eth_getBlockByNumber", [receipt.blockNumber, false]);
if (block.hash !== receipt.blockHash) throw new Error("receipt and canonical block disagree");
const matches = receipt.logs.filter((log) =>
  log.address === token && log.transactionHash === tx && log.blockHash === block.hash
  && log.removed === false && log.topics.length === 3 && log.topics[0] === transfer
  && log.topics[1] === topic(owner) && log.topics[2] === topic(token)
  && /^0x[0-9a-f]{64}$/.test(log.data));
if (matches.length !== 1) throw new Error("expected exactly one USDt transfer to the token contract");

const units = BigInt(matches[0].data);
if (units <= 0n) throw new Error("transfer amount must be positive");
const amount = `${units / 1000000n}.${(units % 1000000n).toString().padStart(6, "0")}`;
const facts = {
  source: endpoint.origin,
  chain: "Ethereum",
  chain_id: 1,
  transaction_hash: tx,
  block_number: BigInt(receipt.blockNumber).toString(),
  block_hash: block.hash,
  observed_finalized_block: BigInt(finalized.number).toString(),
  token: "USDt",
  token_contract: token,
  observed_transfer_from: owner,
  observed_transfer_to: token,
  amount_raw: units.toString(),
  amount_decimal: amount,
  requested_return_address: owner,
  wallet_ownership: "DECLARED_BY_REQUESTER",
  support_url: support,
  receipt,
};
const factsText = JSON.stringify(facts, null, 2) + "\n";
const factsSha256 = createHash("sha256").update(factsText).digest("hex");
const statement = `I, the owner of the address ${owner}, am requesting assistance from Tether to recover the funds sent in TX ${tx} and return them to ${owner}.`;
const request = [
  "Subject: Request for recovery of USDt sent to the Ethereum token contract",
  "",
  statement,
  "",
  `Transaction ID: ${tx}`,
  `Protocol / blockchain: Ethereum (ERC-20)`,
  `Deposit address: ${token}`,
  `Amount: ${amount} USDt`,
  `Requested return address: ${owner}`,
  "",
  "The transaction receipt records a USDt transfer from my address to the USDt token contract.",
  "Please assess whether recovery is possible.",
  "",
  "Additional events leading to the transfer: [complete before submitting]",
  "Actions already taken to recover the funds: [complete before submitting, if applicable]",
].join("\n") + "\n";

await mkdir(values.output);
const save = (name, contents) => writeFile(join(values.output, name), contents, { flag: "wx" });
await Promise.all([
  save("facts.json", factsText),
  save("ownership-statement.txt", statement),
  save("tether-request.txt", request),
]);

if (values["qvac-config"]) {
  const { loadConfig } = await import("./config.mjs");
  const { startWorker } = await import("./worker.mjs");
  const worker = await startWorker(await loadConfig(values["qvac-config"]));
  try {
    const expected = `The Ethereum receipt for ${tx} records ${amount} USDt sent to the USDt token contract ${token}.`;
    const prompt = [
      "Copy the following sentence exactly. Do not copy the SHA-256 line.",
      expected,
      `Facts file SHA-256: ${factsSha256}`,
    ].join("\n");
    const { output, ...evidence } = await worker.infer(prompt);
    const accepted = output.trim() === expected;
    await save("qvac-evidence.json", JSON.stringify({
      facts_sha256: factsSha256,
      scope: "MODEL_DRAFT_ONLY",
      draft_status: accepted ? "MATCHES_VERIFIED_FACTS" : "REJECTED_TEXT_MISMATCH",
      ...(accepted ? { output } : {}),
      ...evidence,
    }, null, 2) + "\n");
  } finally {
    await worker.close();
  }
}

console.log(JSON.stringify({ amount, token: facts.token, output: values.output, support }));
