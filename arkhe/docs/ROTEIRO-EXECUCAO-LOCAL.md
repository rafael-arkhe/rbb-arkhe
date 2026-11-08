# Roteiro de Inicialização e Validação dos Gates Locais (Arkhe-RBB)

Este documento descreve o procedimento padronizado para executar e verificar o conjunto local de testes e validação de protocolo do módulo Arkhe-RBB em ambientes Linux/Unix (Bash).

---

## 1. Escopo e Fronteiras de Validação (P1, P2 e P5)

- **Status no Escopo Executado:** `VERIFIED`
  - Oráculo Python de ABI/Keccak e 8 vetores de Merkle Mountain Range (MMR).
  - 15 testes Rust no crate `arkhe/protocol` (`arkhe-rbb-core`), cobrindo codec de cabeçalho ABI v3 (27 palavras / 864 bytes), extensão e inclusão MMR, assinaturas híbridas Ed25519 + ML-DSA-44 e atestação com quórum 3/4.
  - 25 testes Node.js/EVM no diretório `arkhe/`, cobrindo 15 testes de custódia/fencing/replay, 9 testes EVM com Ganache e solc 0.8.30 validando o contrato `ArkheCheckpointAnchor.sol`, e 1 teste de regressão/opcionalidade do lockfile npm.
  - Verificação de estilo e boas práticas com `cargo clippy --all-features -- -D warnings`.

- **Fronteiras e Limitações Declaradas (Status `PARTIAL` / `NONEXISTENT`):**
  - **Rede RBB de Produção:** Não há deploy efetuado em rede QBFT remota; a EVM executada é estritamente local (Ganache em chainId 1337). Status: `NONEXISTENT`.
  - **Registro Institucional Governado:** `RegistryFixture.sol` é explicitamente uma fixture com setters de teste. O registro governado com HSM em produção permanece pendente. Status: `PENDING`.
  - **Adaptadores de Sistemas de Origem (SAPL, Fabric, Judicial):** São integrações previstas pela especificação, não conectores operacionais entregues. Status: `NONEXISTENT`.

---

## 2. Pré-requisitos do Ambiente

- **Sistema:** Linux x86_64 ou macOS
- **Rust Toolchain:** 1.96.0+ com componentes `cargo` e `clippy`
- **Node.js:** v22+ com `npm` 10.9.9+
- **Python:** 3.12+ com ambiente virtual em `arkhe/.venv`

### Preparação do Ambiente Virtual Python
```bash
python3 -m venv arkhe/.venv
arkhe/.venv/bin/pip install -r arkhe/python/requirements.txt
```

### Instalação de Dependências Node.js
```bash
cd arkhe
npm ci
cd ..
```

---

## 3. Execução Automatizada

Para disparar todos os gates locais em sequência:

```bash
./arkhe/run-tests.sh
```

O script executa em modo fail-closed (`set -euo pipefail`):
1. `arkhe/.venv/bin/python arkhe/python/reference_vectors.py`
2. `cd arkhe/protocol && cargo test --all-features`
3. `cd arkhe && npm test`
4. `cd arkhe/protocol && cargo clippy --all-features -- -D warnings`

Qualquer falha em qualquer etapa interrompe imediatamente a execução e retorna código de saída diferente de zero.
