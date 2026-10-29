import { FORMAT, validateInput } from "./core.mjs";
export function scenario(count = 10) {
  if (!Number.isSafeInteger(count) || count < 1 || count > 1_000_000)
    throw new Error("RESOURCE_LIMIT");
  const start = 1803772800;
  const records = Array.from({ length: count }, (_, i) => ({
    id: "event-" + String(i).padStart(7, "0"),
    movement: "movement-" + i,
    account: "clients",
    delta: "1",
    time: String(start + 10),
    block_number: "1",
    block_hash: "block-1",
    tx: "tx-" + i,
    log_index: "0",
    classification: "clients",
    authorization: "approval-" + i,
  }));
  const scope = {
    network: "evm:1337",
    genesis: "synthetic-genesis",
    asset: "synthetic-token",
    decimals: "0",
    start: String(start),
    end: String(start + 100),
    accounts: [
      { id: "clients", classification: "clients" },
      { id: "own", classification: "own" },
    ],
  };
  const source = (id) => ({
    id,
    owner: "synthetic-" + id,
    origin: id === "chain" ? "synthetic-observer" : "synthetic-system",
    dependencies: [id],
    available: true,
    gaps: [],
    start: scope.start,
    end: scope.end,
    accounts: ["clients", "own"],
    opening: { clients: "0", own: "0" },
    closing: { clients: String(count), own: "0" },
    records: structuredClone(records),
    blocks: [
      {
        number: "1",
        initial_hash: "block-1",
        final_hash: "block-1",
        head: "4",
      },
    ],
  });
  const policy = {
    id: "policy-1",
    valid_from: scope.start,
    valid_until: scope.end,
    max_amount: "10",
    approvals: records.map((r) => ({
      id: r.authorization,
      movement: r.movement,
      approver: "synthetic-controller",
      issued_at: String(start),
      valid_from: scope.start,
      valid_until: scope.end,
      max_amount: "10",
      destination: r.account,
      asset: scope.asset,
      policy_id: "policy-1",
    })),
  };
  const input = {
    format: FORMAT,
    scope,
    sources: {
      chain: source("chain"),
      custody: source("custody"),
      ledger: source("ledger"),
    },
    policy,
  };
  validateInput(input);
  return input;
}
