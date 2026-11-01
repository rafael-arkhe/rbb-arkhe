# Modelo de Ameaças Arkhe AGI-P2P

## Análise de Riscos Residuais e Mitigações

1. **Nó malicioso devolve output adulterado (Petals/OmniNode)**:
   - Mitigação: Verificação em L2/L3 independente do executor; o recibo AFiR é validado contra o `inference_root` ancorado no MMR.
2. **Ataques quânticos futuros contra histórico de evidências**:
   - Mitigação: Checkpoint híbrido distribuído selado no StrongBox com ML-DSA-65 (FIPS 204).
3. **Comprometimento do Fabricante de TEE**:
   - Mitigação: TEE é utilizado estritamente para aceleração, nunca como raiz de confiança exclusiva. A raiz de confiança local é o Titan M2 / StrongBox.
4. **Ataques Sybil na Governança P2P**:
   - Limite declarado: O consenso BFT local protege deliberações em escala de rede de agentes, mas governança global permissionless requer autoridade de identidade / proof-of-personhood.
