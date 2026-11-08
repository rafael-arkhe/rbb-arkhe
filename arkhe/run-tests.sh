#!/usr/bin/env bash
set -euo pipefail

# Arkhe-RBB: Script de inicialização e validação dos gates locais em Linux/Unix
# Requisitos: Python 3 com venv (arkhe/.venv), Rust 1.96.0+, Node.js 22+ e npm 10.9.9+

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "=== [1/4] Executando Oráculo Python de Vetores de Referência ==="
if [ -f "arkhe/.venv/bin/python" ]; then
    PYTHON_BIN="arkhe/.venv/bin/python"
elif command -v python3 >/dev/null 2>&1; then
    PYTHON_BIN="python3"
else
    echo "Erro: Python 3 não encontrado." >&2
    exit 1
fi

"$PYTHON_BIN" arkhe/python/reference_vectors.py

echo "=== [2/4] Executando Testes de Protocolo Rust (arkhe-rbb-core) ==="
(cd arkhe/protocol && cargo test --all-features)

echo "=== [3/4] Executando Testes Node.js e EVM Local (Ganache) ==="
(cd arkhe && npm test)

echo "=== [4/4] Executando Clippy (Avisos tratados como erros) ==="
(cd arkhe/protocol && cargo clippy --all-features -- -D warnings)

echo "=== Todos os gates locais foram executados e aprovados com sucesso! ==="
