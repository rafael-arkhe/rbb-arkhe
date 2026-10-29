# Experimental custody reconciliation pilot

This module implements a first, synthetic-data application of the Arkhe evidence core. It reconciles source snapshots, emits deterministic exceptions, authenticates fixture source declarations and independently recalculates a supplied report offline. It is not a custodian, reserve attestation, certification, or production deployment.

## Run from the repository root (PowerShell)

```powershell
pwsh -NoProfile -File arkhe/run-tests.ps1 -InstallNodeDependencies
node arkhe/custody/evm-pilot.mjs
node arkhe/custody/cli.mjs verify arkhe/test-results/custody/package.json arkhe/test-results/custody/trust.json
node arkhe/custody/benchmark.mjs 1000
```

Prerequisites are the existing pinned Rust 1.96.0, Python requirements and Node/npm environment documented in `../README.md`. The runner builds both Rust examples before Node tests. No new dependencies were added. Verification uses local Rust binaries; it performs no network calls. Run `verify` as a separate process against exported files to distinguish replay from generation. The full test command also executes the preexisting core, EVM and dependency tests.

## Scope and evidence

- Three roles: chain observer, synthetic custody snapshots and synthetic client ledger. Each includes provenance, dependencies, declared coverage and explicit gaps.
- Quantities are BigInt, never floating point; no valuation or currency conversion.
- Rules compare movements and balances, deduplicate IDs, identify classification conflicts, validate authorization bindings and half-open validity, and retain proven divergences alongside coverage gaps.
- Source declarations use Ed25519 with external fixture trust. Manifest commitments use Keccak256. The Rust core recomputes the complete v3 MMR over three file entries and verifies Ed25519 + ML-DSA-44 signatures.
- The local EVM scenario mints 100 test-token credits, observes real receipts and blocks, publishes the real experimental anchor contract, compares event and storage fields, and exports an attested observation.
- A snapshot/revert test creates two actual branches at one height; a changed block reference makes the affected reconciliation non-evaluable.
- Report mutation, an authenticated false report, and a substituted ruleset have distinct failure codes.

The compact single-package transport is specified in `SCHEMA.md`. No arbitrary executable rules or archive paths are accepted. A new format requires a new version. The ruleset digest identifies declared semantics; use a pinned Git revision to identify the implementation as well.

## Limits and deviations from the broader v0.2 design

The domain is synthetic client credits from a minimal mint-only test token. This is not a full double-entry accounting ledger, ERC-20 production asset, or adapter for real custodians. Token balances are measured in token units; native gas belongs to the operator and is excluded from the declared token scope. Data generators are deterministic, with scenario identifier 20261029; no random sampling is involved.

The custody package uses full single-batch frontier recomputation instead of transporting compact inclusion/extension proofs. Core proof tests remain separate. Continuous batches and private-organization protocol semantics are not implemented. The institutional `power` value is fixture-only and does not classify a bank as a branch of government.

All signing seeds are public. `LOCAL_OPERATOR_ATTESTED` exercises signature validation of a test operator's statement; it does not establish adversarial authenticity or independently prove chain consensus. A true fixture signature must not be presented as independent ancoragem. The exported observation is not an Ethereum state proof. Omitted wallets or operations absent from all sources remain undiscoverable.

Build reproducibility, clean-environment execution and independent reproduction are distinct. Lockfiles and scripts are provided; no bit-for-bit build claim or independent third-party reproduction is made. Results must identify environment and revision.

The 1000-record-per-source memory benchmark has one warmup and three measurements. Optional 100000 and 1000000 levels have no qualification claim. Its timings exclude chain publication and hybrid signing, which the EVM integration exercises separately; it is not a chain throughput benchmark. A process supervisor should enforce a 300-second timeout for optional levels. Test results go to ignored `../test-results/custody/`.

## Unanchored fixtures

```powershell
node arkhe/custody/cli.mjs generate arkhe/test-results/custody/scenario.json 1000
node arkhe/custody/cli.mjs trust-fixture arkhe/test-results/custody/trust.json
node arkhe/custody/cli.mjs verify arkhe/test-results/custody/scenario.json arkhe/test-results/custody/trust.json
```

Unanchored fixtures explicitly report `NOT_PROVIDED`; passing reconciliation never implies that an anchor exists.
