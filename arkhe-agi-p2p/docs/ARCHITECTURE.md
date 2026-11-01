# Arquitetura Arkhe AGI-P2P: Pilha de Verificação em Profundidade

O ecossistema AGI-P2P materializa a convergência entre sistemas inteligentes descentralizados e verificação criptográfica sem autoridade central de pesos ou inferência.

A arquitetura organiza os projetos complementares em quatro camadas em profundidade:

```text
┌─────────────────────────────────────────────────────────────────────┐
│ L4 — GOVERNANÇA E INCENTIVOS                                        │
│ OpenCLAW-P2P (BFT + reputação) · A.N.N. (sem autoridade central)   │
├─────────────────────────────────────────────────────────────────────┤
│ L3 — EVIDÊNCIA LOCAL E CONSISTÊNCIA HISTÓRICA                       │
│ ArkheOS Mobile: MobileWER · MMR selado pelo StrongBox               │
│ Binding: attestation_blob ↔ MobileWER · OfflineVerificationBundle   │
├─────────────────────────────────────────────────────────────────────┤
│ L2 — PROVENIÊNCIA DA INFERÊNCIA                                     │
│ IETF SPICE (inference_root) · AFiR (ML-DSA-65, TEE-free)           │
│ ZKML (opcional, verificação forte) · TEE (aceleração)               │
├─────────────────────────────────────────────────────────────────────┤
│ L1 — DISTRIBUIÇÃO E CONSENSO                                        │
│ Petals (camadas) · Hyperspace (Mysticeti DAG)                       │
│ OmniNode (BLAKE3 → CIDv1, SUM Chain)                                │
└─────────────────────────────────────────────────────────────────────┘
```
