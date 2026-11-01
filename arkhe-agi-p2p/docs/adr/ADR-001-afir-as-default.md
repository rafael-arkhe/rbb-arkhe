# ADR-001: AFiR como Mecanismo Padrão de Proveniência

## Status
Aceito

## Contexto
Dispositivos móveis e nós distribuídos necessitam de proveniência criptográfica sem o overhead de 1000x do ZKML nem a dependência exclusiva de fabricantes de TEE.

## Decisão
Adotar AFiR (ML-DSA-65) como perfil padrão de proveniência, gerando recibos compactos de 384 bytes com ~0.785 ms de overhead, compatíveis com a árvore SPICE.
