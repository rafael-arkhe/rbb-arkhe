# Execução do módulo autocontido no fork

Preparado em 28/10/2026 sobre o commit `a3069c60eabb0f99535fe904b9485de193a060e4` de `rafael-arkhe/rbb-arkhe`, na branch `codex/arkhe-evidence-v3`.

## O que mudou na portabilidade

O núcleo foi extraído para `arkhe/protocol`, crate `arkhe-rbb-core`, sem dependência de outros projetos do workspace original. Imports, caminhos dos vetores e localização do assinador de teste foram adaptados. Os quatro módulos de protocolo foram copiados sem mudança de conteúdo, conforme [proveniência](../PROVENANCE.json). A estrutura original RBB é preservada; apenas uma referência à camada Arkhe foi acrescentada ao README da raiz.

As especificações/revisão em `docs/` registram o desenho e achados anteriores; referências narrativas ao workspace original são históricas, não requisitos de instalação. O perfil MMR novo tem identidade própria e não reinterpreta raízes anteriores. O anexo privado não é distribuído; seu inventário preserva hashes e posições, sem caminho de usuário.

## Verificações reproduzíveis

O [runner](../run-tests.ps1) executa, nesta ordem:

1. Instalação Node por lockfile, quando solicitada.
2. Oráculo Python independente de ABI/Keccak e oito raízes MMR.
3. Testes próprios da crate: sete testes de codec/MMR e oito de assinatura/confiança.
4. Compilação do exemplo Rust com assinaturas híbridas reais de teste.
5. Nove testes de compilação e execução Solidity em EVM local, incluindo o mesmo checkpoint assinado em Rust.
6. Clippy da biblioteca, testes e exemplos com avisos tratados como erros.

O arquivo `arkhe/test-results/results.json`, gerado pelo runner e ignorado pelo Git, registra êxito/falha e caminhos dos logs. Este checkout tem 15 testes Rust próprios; os 82 testes citados no relatório do workspace original incluíam outras funcionalidades que não fazem parte deste módulo e não foram copiados.

Na execução local desta preparação, as seis etapas passaram: 15 testes Rust e 9 EVM aprovados, oráculo Python conferido, exemplo híbrido compilado e Clippy sem avisos. O [resumo versionado](TEST-RESULTS.json) registra essa contagem; os logs completos ficam no diretório local de resultados.

O workflow `.github/workflows/arkhe-tests.yml` repete a execução em Windows. Os resultados registrados neste documento são da validação local anterior à publicação; a execução remota de CI é apresentada separadamente nos checks do pull request. Nenhuma implantação em rede RBB é executada pelo workflow.

## Alcance

São testes locais: solc 0.8.30, alvo Shanghai e EVM chain ID 1337. Não houve deploy na RBB nem conexão SAPL/Fabric. O `RegistryFixture` tem setters públicos e é exclusivamente de teste. O anchor registra autorização e compromissos, sem verificar ML-DSA on-chain; a aceitação criptográfica ocorre no verificador Rust sob trust context externo.

Permanecem pendentes registro institucional governado, persistência/outbox com recuperação de falhas, assinador isolado/HSM, testemunhas operacionais e integração aos sistemas/rede de homologação. O módulo não declara prontidão de produção nem aprovação institucional.
