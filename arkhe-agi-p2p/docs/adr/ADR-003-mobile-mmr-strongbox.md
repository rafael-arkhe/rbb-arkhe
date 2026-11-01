# ADR-003: MMR Local Selado com StrongBox

## Status
Aceito

## Contexto
O hardware StrongBox (Pixel/Titan M2) suporta ML-DSA a partir do Android 17, mas não suporta Ed25519 (TEE-only).

## Decisão
Distribuir as assinaturas do checkpoint híbrido: ML-DSA no StrongBox e Ed25519 em TEE/Software, encapsuladas em envelope unificado.
