import { performance } from "node:perf_hooks";
import { cpus, totalmem, platform, release } from "node:os";
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  canonical,
  makePackage,
  replay,
  testTrust,
  parseCanonical,
} from "./core.mjs";
import { scenario } from "./scenarios.mjs";
const count = Number(process.argv[2] ?? 1000);
if (![1000, 100000, 1000000].includes(count))
  throw new Error("Unsupported benchmark level");
const samples = [];
for (let run = 0; run < 4; run++) {
  const start = performance.now(),
    input = scenario(count),
    generated = performance.now();
  const pkg = makePackage(input),
    packaged = performance.now(),
    bytes = Buffer.from(canonical(pkg)),
    encoded = performance.now();
  const result = replay(parseCanonical(bytes), testTrust()),
    end = performance.now();
  if (result.report.reconciliation !== "consistent_in_scope")
    throw new Error("Benchmark did not reconcile");
  if (run)
    samples.push({
      generation_ms: (generated - start).toFixed(3),
      package_signing_ms: (packaged - generated).toFixed(3),
      serialization_ms: (encoded - packaged).toFixed(3),
      offline_parse_replay_ms: (end - encoded).toFixed(3),
      bytes: String(bytes.length),
      process_lifetime_peak_rss_kib: String(process.resourceUsage().maxRSS),
    });
}
const metric = (key) =>
  samples
    .map((x) => Number(x[key]))
    .sort((a, b) => a - b)[1]
    .toFixed(3);
const report = {
  format: "arkhe-custody-benchmark/1",
  records_per_source: String(count),
  total_movement_records: String(count * 3),
  seed: "20261029",
  measurements: samples,
  median: {
    generation_ms: metric("generation_ms"),
    package_signing_ms: metric("package_signing_ms"),
    serialization_ms: metric("serialization_ms"),
    offline_parse_replay_ms: metric("offline_parse_replay_ms"),
  },
  environment: {
    node: process.version,
    cpu: cpus()[0].model,
    logical_cpus: String(cpus().length),
    ram_bytes: String(totalmem()),
    os: platform() + " " + release(),
    disk: "not_measured_memory_workload",
  },
  limitations: [
    "Synthetic snapshots; not EVM throughput.",
    "Peak RSS is process lifetime, not isolated per phase.",
    "100000 and 1000000 levels have not been qualified unless separately reported.",
  ],
};
const out = fileURLToPath(new URL("../test-results/custody", import.meta.url));
mkdirSync(out, { recursive: true });
writeFileSync(resolve(out, "benchmark-" + count + ".json"), canonical(report));
console.log(JSON.stringify(report));
