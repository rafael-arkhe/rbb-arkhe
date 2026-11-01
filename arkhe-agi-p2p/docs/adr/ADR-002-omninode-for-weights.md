# ADR-002: OmniNode para Content-Addressing de Pesos

## Status
Aceito

## Contexto
Garantir que os modelos recebidos via P2P correspondam exatamente aos declarados.

## Decisão
Usar o padrão do OmniNode: particionamento por transformer blocks com identificação BLAKE3 → CIDv1.
