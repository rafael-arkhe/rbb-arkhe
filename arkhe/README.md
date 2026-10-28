# Arkhe — camada experimental de evidências para a RBB

Núcleo autocontido do candidato `arkhe-evidence/3`, revisão 2: cabeçalho ABI, envelope híbrido Ed25519 + ML-DSA-44, MMR BLAKE3, verificação offline de atestação 3/4 e contrato experimental de ancoragem.

O código reside em `arkhe/` e usa sua própria crate, lockfiles e testes. Não requer o workspace original `D:\arkhe`, não altera o consenso da rede e não pressupõe precompile PQ disponível na RBB.

| Diretório | Conteúdo |
|---|---|
| `protocol/` | Crate `arkhe-rbb-core`, testes adversariais e assinador de fixture |
| `contracts/` | Anchor Solidity e registro **exclusivamente de teste** |
| `fixtures/` | Vetores públicos de ABI/Keccak/MMR |
| `python/` | Oráculo independente e dependências fixadas |
| `tests/` | Compilação Solidity e execução em EVM local |
| `docs/` | Revisão, especificação, arquitetura e limites da execução |

## Executar a partir de um checkout deste repositório

Com Rust/rustup, Node/npm, Python e PowerShell 7 instalados:

```powershell
python -m venv arkhe/.venv
arkhe/.venv/Scripts/python.exe -m pip install -r arkhe/python/requirements.txt
pwsh -NoProfile -File arkhe/run-tests.ps1 -Python arkhe/.venv/Scripts/python.exe -InstallNodeDependencies
```

Comandos independentes, válidos também em outros sistemas com os executáveis correspondentes:

```text
python arkhe/python/reference_vectors.py
cargo +1.96.0 test --manifest-path arkhe/protocol/Cargo.toml --features rbb-pqc --locked
cargo +1.96.0 build --manifest-path arkhe/protocol/Cargo.toml --example rbb_sign_fixture --features rbb-pqc --locked
npm --prefix arkhe test
```

O teste integrado espera o exemplo Rust em `arkhe/protocol/target/debug/examples/`; se `CARGO_TARGET_DIR` foi alterado, informar `ARKHE_RBB_FIXTURE_BIN`. O runner falha em divergência dos vetores, não os regenera. Dependências Node e Python são ferramentas locais de teste.

## Estado e limites

[Resultados e alcance](docs/EXECUCAO.md), [arquitetura v0.2](docs/ARKHE-RBB-ARQUITETURA-v0.2.md), [especificação](docs/ARKHE-RBB-ESPECIFICACAO-TECNICA-v0.2.md) e [revisão de segurança](docs/ARKHE-RBB-REVISAO-SEGURANCA-2026-10-28.md).

O anchor registra autorização/compromissos e não valida ML-DSA na EVM. `RegistryFixture.sol` tem setters públicos e **não deve ser implantado como registro institucional**. Administração governada, registro completo, persistência PostgreSQL/outbox, HSM, testemunhas operacionais e integrações reais SAPL/Fabric/RBB permanecem pendentes. Trust context é entrada independente do host; não pode ser aceito do próprio pacote avaliado. O exemplo de assinatura tem seeds públicas de teste.

O código deste módulo mantém seus avisos de licença; a licença e os documentos herdados do repositório RBB permanecem na raiz. [Proveniência da portabilidade](PROVENANCE.json) identifica os arquivos de origem e as adaptações locais.
