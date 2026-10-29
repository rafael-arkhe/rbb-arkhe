# Custody pilot schema, version 1

The executable schema is `validateInput` in `core.mjs`; unknown fields are rejected. This document describes its exact record shapes. All domain integers are canonical decimal strings. The canonical parser rejects JSON numbers, duplicate properties, alternate whitespace/escapes, non-ASCII strings, unknown input properties, excessive nesting and excessive record counts. Objects have lexicographically sorted keys. Arrays preserve their declared order; record order is authenticated, while reconciliation deduplicates and sorts by stable ID.

`Input = {format, scope, sources, policy}`. `format` is `arkhe-custody/1`.

`Scope = {network, genesis, asset, decimals, start, end, accounts}`. `accounts` contains unique `{id, classification}` objects; classification is `own` or `clients`. Time range is `[start,end)`.

`Sources = {chain, custody, ledger}`. Each source is `{id, owner, origin, dependencies, available, gaps, start, end, accounts, opening, closing, records, blocks}`. `id` must match its role. Dependencies, gaps and accounts are string arrays. `opening` and `closing` map exactly the declared scope accounts to unsigned quantities. `available` is boolean.

`Record = {id, movement, account, delta, time, block_number, block_hash, tx, log_index, classification, authorization}`. Delta is signed; all other integers are unsigned. In the EVM fixture there is one client credit per mint event. Real double-entry and omnibus custody adapters are not implemented by this profile.

`Block = {number, initial_hash, final_hash, head}`. These are authenticated source observations, not consensus proofs. A chain record must reference an observed block/hash. Confirmation depth is head minus block number; the threshold is 3.

`Policy = {id, valid_from, valid_until, max_amount, approvals}`. `Approval = {id, movement, approver, issued_at, valid_from, valid_until, max_amount, destination, asset, policy_id}`. Authorizations are synthetic declarations committed in the checkpoint, not independent signatures of real approvers.

`Package = {format,input,report,rules,sources,manifest,anchor}`. `sources` contains signed `{body,signature}` documents for all three sources. Each body is `{scope_digest,source}`. Signatures are Ed25519 over a source-domain prefix and canonical body. `manifest = {format,files,policy_digest,scope_digest}`. Files are ordered input.json, report.json, rules.json, each with `{path,digest}`. They are logical entries embedded in the package; no archive extraction occurs.

`anchor` is null for unanchored scenarios, or a signed `{body,signature}` document in the anchor domain. The EVM execution body contains `{manifest_digest,checkpoint,observation}`. Checkpoint contains `{header,ed,ml}`: header integers are decimal strings, byte fields are lowercase hex with `0x`, booleans remain booleans, `ed` is signature hex without prefix, and `ml` is signature hex in chunks up to 2048 characters. The Rust bridge converts this transport to the existing v3 ABI envelope and verifies both signatures and full MMR recomputation. The pilot supports one batch of three manifest entries; extension and compact inclusion proofs remain in the existing core suite, not the custody transport.

External trust contains `{format,rulesets,sources,anchor,test_keys_public}`. It must be supplied separately. Its source public keys and anchor public key are SPKI DER hex. Fixture generation is explicit (`trust-fixture` command). Source/anchor seeds are public test material; the verifier has no production authentication claim.

The offline output separates `integrity`, `report` and `anchor`. Hybrid verification appears separately as `checkpoint` when a checkpoint is present. `chain_observation_independently_verified` remains false. Consistency is scoped to the data and rules, never solvency.
