# L3 — Evidência Local (ArkheOS Mobile)

## MobileWER
- Ancoragem criptográfica da ação ao hardware seguro:
  `device_attestation_hash = H(attestation_blob || first_wer_nonce || app_measurement)`
- Captura de input_digest, output_digest, policy_digest e timestamps.

## MMR Local Selado
- Merkle Mountain Range append-only com prova de inclusão e consistência.
- Envelope híbrido distribuído: **ML-DSA-65 no StrongBox** (Titan M2, Android 17) + **Ed25519 no TEE/Software**.

## Verificação Offline
- `OfflineVerificationBundle` permite que terceiros detectem reescritas históricas sem contactar nenhum servidor.
