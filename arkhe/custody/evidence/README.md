# Execution evidence — 29 October 2026

This directory contains synthetic test data only. `package.json` is an exported evidence package, not an npm dependency manifest. Public fixture keys are intentional.

## Verified in this execution

| Check | Result |
|---|---|
| Isolated directory initially without node_modules or Rust target | Dependencies installed with npm 10.9.9; Rust compiled from source |
| Final full runner | Windows, Node 24.13.0, npm 10.9.9, Rust 1.96.0; passed |
| Node suite | 25 passed: 10 existing, 14 custody unit/negative scenarios, 1 custody EVM integration |
| Rust suite | 15 passed |
| Independent Python reference vectors | Passed |
| Clippy with warnings denied | Passed |
| Local EVM execution | 100 test-token mint transfers; checkpoint event and storage checked |
| Separate-process offline replay | Both hybrid signatures valid; complete single-batch MMR recomputed |
| Actual alternate local EVM branch | Replacement hash detected; affected coverage insufficient |
| Synthetic memory benchmark | 1000 movement records per source, 3000 total; three measurements after warmup |

The initial fresh-directory Node suite also passed under Node 22.23.0 via npm exec. The final runner explicitly uses `node.exe` because an extensionless Node launcher supplied by npm exec could be resolved incorrectly by PowerShell for direct stages. No blank stage output was accepted as evidence for the pilot: the final exported files and replay output were checked.

Median local benchmark: generation 45.833 ms; package/signing 255.938 ms; serialization 59.128 ms; offline parse/replay 376.767 ms. Package size: 1,639,294 bytes. Detailed samples and hardware are in `benchmark-1000.json`. This benchmark excludes hybrid signing and EVM publication; it is not a throughput or production SLO claim. Higher benchmark levels were not qualified.

## Replay the published EVM package

After building the Rust examples with the documented pinned toolchain:

```powershell
node arkhe/custody/cli.mjs verify arkhe/custody/evidence/package.json arkhe/custody/evidence/trust.json
```

Compare with `result.json`. `execution.json` records Solidity source digests and the result of the separate reorg scenario. `source-hashes.json` identifies implementation files used for this report. The surrounding Git commit pins the rest of the source tree and dependency lockfiles.

## What this does not establish

This is execution by the author in a new directory on the same host, with shared installed toolchains and package caches. It is not an independent third-party reproduction, isolated operating system, bit-for-bit reproducible build, live RBB deployment or regulatory certification. The trust keys are public and synthetic; the local attestation does not independently prove consensus. The test token is minimal and mint-only. See the module README for scope deviations from the broader proposed specification.
