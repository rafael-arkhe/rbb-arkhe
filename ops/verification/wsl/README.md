# Verificação Arkhe em um contêiner WSL2

O script usa o Podman rootless do WSL2 em um host Windows x64. Use um
checkout confiável, com arquivos rastreados limpos e sem arquivos
adicionais que possam influenciar o build. A execução monta o checkout
e o cache do Cargo em modo somente leitura. O contêiner roda sem rede,
com uma imagem local fixada pelo ID e um volume descartável para o build.

## Rodar a suíte e compilar o CLI

No PowerShell, a partir da raiz Arkhe:

```powershell
wsl --list --verbose
podman machine list
# Se a máquina estiver parada:
podman machine start arkhe-runtime
./scripts/verify-afir-wsl.ps1 -Source D:/arkhe-afir-recheck
```

`-Source` aponta para o checkout que será testado. Use `-Connection` para
outra máquina Podman local e `-CargoRegistry` para outro cache Windows
com os diretórios `cache` e `index` já preenchidos. O perfil usa o Rust
fixado em `rust-toolchain.toml`. Uma imagem alternativa deve conter essa
versão e ser passada por ID completo em `-ImageId`.

A suíte roda os crates `arkhe-verify-core`, `arkhe-verify-cli` e
`arkhe-verify-wasm` no alvo nativo Linux. Em seguida, compila
`arkhe-verify`, confere a inicialização do CLI e regenera o vetor
`afir-retrieval-v1.json`. O resultado deve coincidir byte a byte com o
vetor do checkout. A implementação AFiR desse perfil vive em
`arkhe-verify-core`. O formato binário de recuperação AFiR v1 tem
3.449 bytes, com 140 bytes de payload e 3.309 bytes de assinatura
ML-DSA-65. Esses tamanhos constam no perfil Rust.

## Compilar o módulo WASM

Prepare a imagem uma vez, com acesso à rede para obter o alvo Rust:

```powershell
podman --connection arkhe-runtime build --pull=never -t localhost/arkhe-verifier-wasm:rust-1.96.0 ops/verification/wsl
$id = podman --connection arkhe-runtime image inspect localhost/arkhe-verifier-wasm:rust-1.96.0 --format '{{.Id}}'
./scripts/verify-afir-wsl.ps1 -Source D:/arkhe-afir-recheck -ImageId $id -BuildWasm
```

O segundo comando de verificação também compila para
`wasm32-unknown-unknown` e imprime o SHA-256 do módulo bruto. Para
guardar o módulo, passe `-WasmOutput` com o caminho de um diretório novo.

## Verificar um bundle no Windows

Exporte o módulo do WSL2 e gere os bindings com a versão do
`wasm-bindgen` fixada no `Cargo.lock` do checkout. Use o executável
Windows oficial dessa versão. No checkout do ensaio, a versão é
0.2.129.

```powershell
./scripts/verify-afir-wsl.ps1 -Source D:/arkhe-afir-recheck -ImageId $id -BuildWasm -WasmOutput D:/arkhe-wasm-windows
wasm-bindgen --target nodejs --out-dir D:/arkhe-wasm-windows/node --out-name arkhe_verify D:/arkhe-wasm-windows/arkhe_verify_wasm.wasm
node ops/verification/wsl/verify-wasm-windows.cjs test D:/arkhe-wasm-windows/node/arkhe_verify.js D:/arkhe-afir-recheck/verification/vectors/afir/afir-bundle-v1.json
```

O teste usa os vetores AFiR de inferência e checkpoints. Para verificar
evidência real, troque `test` por `verify` e passe o caminho do bundle
e o arquivo separado de chaves confiáveis. Obtenha as chaves por um
canal confiável. O consumidor Node lê os arquivos locais e retorna o
parecer ou um código de erro do verificador.

## Verificar o caminho CUDA

Em um host Windows com GPU NVIDIA e driver compatível com WSL2, rode
o probe em uma distribuição Linux com Python 3.10 ou superior. Exemplo
para este checkout e a distribuição Ubuntu:

```powershell
wsl -d Ubuntu -- /usr/bin/python3 /mnt/d/arkhe-self-build-afir/ops/verification/wsl/smoke-cuda.py
```

O probe carrega o driver CUDA exposto no WSL, compila o kernel PTX no
driver e executa uma camada densa ReLU de três saídas. Ele lê o resultado
da GPU, compara com a referência CPU e retorna erro se a execução ou a
comparação falhar. O teste usa pesos sintéticos e dispensa CUDA Toolkit,
download de modelos e serviços externos.

`tpmtool getdeviceinformation` no Windows informa se o TPM está disponível.
A prova de boot e a vinculação da computação WSL exigem um protocolo de
atestação próprio. A integração P2P também permanece uma etapa separada.

## Executar um modelo e selar a transcrição

O ensaio usa o modelo pré-treinado TinyStories 260K em GGUF e os
binários Linux x64 do llama.cpp b11801 com CUDA 12.8. Prepare os pesos,
extraia os dois pacotes do release em diretórios locais e confira os
SHA-256 publicados antes de executar.

No WSL2, a partir da raiz Arkhe, configure `runtime` com o caminho do
`llama-completion`, `cuda_libs` com o diretório das bibliotecas CUDA
do release, `model` com o GGUF e `run` com um diretório novo:

```bash
python3 ops/verification/wsl/run-cuda-inference.py --runtime "$runtime" --cuda-libs "$cuda_libs" --model "$model" --run-dir "$run"
cargo run --locked --offline -q -p arkhe-verify-core --example afir_local_inference -- seal "$run" "$model" > "$run/receipt.json"
```

O script exige que o runtime reporte camadas descarregadas em CUDA.
O recibo AFiR de inferência liga o GGUF inteiro, tratado como um chunk,
aos bytes de `input.json` e `output.json`. O fingerprint aponta para
um descritor local do modelo. O relatório CUDA está dentro da saída
assinada como observação de software. A chave ML-DSA-65 é efêmera e de
laboratório. O recibo registra a atestação de hardware como
`NOT_VERIFIED`.

No Windows, usando os mesmos arquivos e uma chave pública do ensaio
fixada por um canal confiável, rode:

```powershell
cargo run --locked --offline -q -p arkhe-verify-core --example afir_local_inference -- verify $run $model $trustedKey
```

O arquivo `$trustedKey` contém a chave pública em hexadecimal.
O verificador usa essa chave para autenticar o recibo e recalcula os
compromissos do modelo, da entrada e da saída.

### Fechar o primeiro ciclo de autoconstrução

1. **Validar a candidata N+1 em outro nó.** Reconstruir a partir do commit fixado, comparar os artefatos e emitir um parecer independente. Windows, WSL2 e Podman já foram usados no mesmo host físico.
2. **Autorizar a promoção de N+1.** Registrar a decisão `Allow` ancorada e obter as assinaturas do limiar de delegados, conforme a política vigente.

### Fazer a Arkhe conduzir o próximo ciclo

3. **Conectar um modelo real ao planejador.** O TinyStories já rodou em CUDA com recibo AFiR. A seleção de tarefas do core foi validada com um modelo simulado. Falta ligar um modelo capaz de propor alterações ao loop do AGI Coder.
4. **Testar uma alteração inédita proposta pela Arkhe.** Fixar os critérios antes da execução, aplicar o patch em um contêiner descartável e testar a falha, a correção e os controles de adulteração. O executor atual já repetiu uma correção conhecida.
5. **Produzir N+2 a partir de N+1.** A versão promovida deve conduzir a mudança e entregar a candidata, os artefatos e a trilha de decisões para nova validação independente.

### Completar o nó WSL2 no protocolo

6. **Vincular a execução à atestação do host.** Definir como o Windows gera uma prova TPM com desafio fresco e como o verificador a vincula à sessão e ao checkpoint AFiR. Hoje o resultado é `NOT_VERIFIED`.
7. **Testar a troca P2P entre dois nós.** Distribuir um chunk GGUF e seu manifesto, verificar o CID no receptor e trocar recibos e checkpoints auditáveis offline.
