# Arkhe — piloto de conciliação verificável de custódia

Versão: 0.2 — decisões do piloto fechadas para implementação
Data: 29 de outubro de 2026

## Objetivo

Permitir que um terceiro reproduza, offline, a conciliação de dados de custódia de um perímetro declarado, confira sua integridade e identifique divergências e limitações. O piloto usa dados sintéticos, uma rede EVM local e um token de teste. Não movimenta fundos reais nem estabelece conclusão de solvência, titularidade jurídica ou conformidade regulatória.

O módulo é uma aplicação do núcleo de evidências Arkhe. O núcleo permanece independente do domínio de custódia e do ativo utilizado. OUSD não é dependência do piloto.

## Fronteira de confiança

Uma assinatura autentica uma declaração de uma fonte sob uma chave confiável; não demonstra a veracidade ou completude dessa declaração. Merkle/MMR protege propriedades dos dados comprometidos; não descobre operações ausentes de todas as fontes.

As fontes sintéticas são controladas pelo executor do teste. Elas simulam papéis distintos, mas não demonstram independência operacional de instituições reais. As chaves de confiança do verificador são fornecidas externamente ao pacote; nunca aceitas apenas porque estão incluídas nele.

A ancoragem EVM local demonstra integração com o contrato experimental. Não representa implantação ou homologação na RBB. O verificador não presume que um recibo exportado, sozinho, prova consenso da rede. A modalidade de confiança na âncora deve ser declarada e verificada separadamente.

## Entradas obrigatórias

1. Manifesto de perímetro: organização, rede e gênese, contrato do ativo, escala decimal, contas conhecidas, classificação própria/clientes, período e bloco inicial/final com hashes.
2. Manifesto de fontes: identificador, responsável, origem, método de coleta, autenticação, dependências compartilhadas, cobertura esperada e observada e lacunas declaradas.
3. Observador EVM: saldos nos limites do período, eventos canônicos do token, identificadores de transação e logs, blocos e estado das transações.
4. Sistema de custódia simulado: movimentações, contas, classificação e vínculo com transações.
5. Razão de clientes sintético: saldos iniciais/finais e lançamentos com identificadores estáveis e referências às movimentações pertinentes.
6. Autorizações e políticas versionadas: ator, operação, destinatário, ativo, valor máximo, vigência, aprovação e vínculo inequívoco com a execução.

Valores são inteiros na menor unidade do ativo, serializados como strings decimais canônicas. Não usar ponto flutuante. Comparar quantidades por ativo; valoração monetária e câmbio ficam fora do piloto.

Taxas de gás denominadas no ativo nativo devem ser registradas separadamente das quantidades do token. Eventuais taxas de negócio precisam de lançamentos explícitos.

## Normalização e identificação

Eventos on-chain são identificados por rede, hash de transação e índice do log, preservando também bloco e hash de bloco para detectar reorganização. Lançamentos off-chain usam identificador da fonte e identificador nativo. Duplicatas idênticas não somam duas vezes; mesmo identificador com conteúdo diferente produz conflito.

A serialização do pacote deve ser determinística, versionada e definida antes da implementação. Arquivos, registros, ordenação, representação de ausências e limites de tamanho devem ter regras explícitas. O verificador rejeita versões desconhecidas e dados ambíguos.

## Regras mínimas

- Recalcular saldo inicial + entradas − saídas = saldo final, por conta e ativo, considerando lançamentos explícitos aplicáveis.
- Confrontar movimentações observadas na rede com registros de custódia e seus vínculos com o razão de clientes. Transferências internas podem exigir correspondência entre vários lançamentos; não presumir relação universal de um para um.
- Comparar saldos atribuídos a clientes e ativos classificados no mesmo perímetro, sem concluir solvência a partir da comparação.
- Detectar duplicidade, colisão de identificadores, classificação conflitante e referências ausentes.
- Verificar autorização vinculada à operação, vigência e limite conforme semântica temporal definida para o cenário.
- Detectar lacunas de cobertura conhecidas e dependências compartilhadas declaradas.
- Ordenar e identificar exceções de forma determinística.

As regras devem possuir versão e digest, e ser reexecutáveis por um verificador com conjunto de regras permitido externamente. Não executar código arbitrário fornecido no pacote. Um digest identifica regras; não demonstra que elas são corretas.

## Resultados independentes

Cada relatório deve apresentar dimensões separadas:

| Dimensão | Estados mínimos |
|---|---|
| Integridade e autenticidade | válida / inválida / não verificável |
| Cobertura | suficiente para as regras declaradas / insuficiente / desconhecida |
| Conciliação | consistente no perímetro / divergente / não avaliável |
| Autorização | válida nas operações avaliadas / violação / não avaliável |
| Âncora | verificada sob o modelo declarado / inválida / não verificada |

Uma lacuna não apaga divergências já demonstradas. Ausência de divergências com cobertura insuficiente não produz aprovação. Dados não autenticados podem ser diagnosticados, mas não sustentam conclusão autenticada.

## Pacote offline

- Manifesto de versão, perímetro e fontes.
- Entradas normalizadas e evidências de origem, com hashes e assinaturas quando aplicáveis.
- Política, autorizações e identificação das regras suportadas.
- Relatório e exceções esperadas, para comparação com o resultado recalculado.
- Compromissos, provas MMR e envelope híbrido compatíveis com o perfil do núcleo Arkhe.
- Evidência de ancoragem e declaração do modelo de confiança.
- Lista de limitações, lacunas e dependências.

O manifesto deve vincular todas as entradas, regras e saídas ao compromisso publicado. O verificador confere essa vinculação, reexecuta as regras e compara os resultados. A verificação do checkpoint e a verificação das provas MMR são etapas explícitas e distintas.

## Critérios de aceitação

| Cenário | Resultado obrigatório |
|---|---|
| Base consistente e coberta | Mesmos resultados em geração e reexecução offline |
| Transferência omitida na custódia, presente no observador | Divergência com referência ao evento sem correspondência |
| Evento idêntico duplicado | Duplicidade relatada; totais não inflados |
| Identificador repetido com conteúdo diferente | Conflito explícito |
| Classificação própria/clientes conflitante | Exceção; sem classificação automática silenciosa |
| Operação acima do limite | Violação da política vinculada |
| Fonte indisponível | Cobertura insuficiente e regras afetadas não avaliáveis |
| Divergência e fonte indisponível simultâneas | Preservar a divergência e declarar a lacuna |
| Arquivo alterado após compromisso | Falha de integridade, sem conclusão autenticada |
| Chave desconhecida ou assinatura inválida | Rejeição de autenticidade |
| Relatório adulterado | Reexecução ou compromisso detecta incompatibilidade |
| Política/regras trocadas | Falha na vinculação ou rejeição de perfil |
| Carteira ausente de todas as fontes | Limitação expressa; nenhuma alegação de descoberta |
| Bloco de referência substituído | Conflito de referência; cobertura/âncora reavaliadas |

O executor dos testes conhece o cenário oculto da carteira omitida; esse conhecimento não é atribuído ao verificador. O teste demonstra uma limitação e não uma capacidade de detecção.

## Entregáveis e sequência

1. Schemas, vetores sintéticos e manifesto de ameaças.
2. Normalização, regras determinísticas e gerador de exceções.
3. Empacotamento autenticado e verificador offline sem acesso à rede.
4. Integração com MMR, assinaturas e âncora EVM experimental.
5. Execução automatizada dos cenários, com comandos, versões e resultados publicados.

Conclusão do piloto exige reprodução em ambiente limpo, incluindo todos os cenários negativos. Desempenho deve ser medido com conjunto e hardware declarados; metas comerciais de escala dependem dessas medições.

## Posicionamento e referências

Formulação: “Arkhe fornece evidências verificáveis para conciliação de custódia e avaliação de segregação patrimonial, vinculando políticas, autorizações, movimentações observadas e exceções em um histórico resistente a alterações posteriores.”

Suporte à avaliação não equivale a certificação ou aceitação pelo supervisor. Alegações sobre concorrentes e demanda comercial permanecem hipóteses até pesquisa e validação específicas.

Referência regulatória contextual: Resolução BCB 589/2026, que altera a Resolução 520/2025. Os dispositivos sobre informações de saldos, custódia, provas de reservas por ativo e staking têm efeitos previstos para 1º de janeiro de 2027, observados os destinatários e a regulamentação específica. A resolução, isoladamente, não fundamenta promessa de periodicidade diária nem obriga uso do Arkhe ou da RBB.

Fonte oficial consultada em 29/10/2026: https://www.bcb.gov.br/estabilidadefinanceira/exibenormativo?numero=589&tipo=Resolu%C3%A7%C3%A3o+BCB

## Decisões normativas v0.2

Esta seção prevalece sobre formulações genéricas anteriores. As decisões são requisitos propostos, não funcionalidades implementadas ou resultados medidos.

### 1. Contrato e confiança na âncora

Usar sem simplificação `arkhe/contracts/ArkheCheckpointAnchor.sol` e `arkhe/contracts/fixtures/RegistryFixture.sol`, do checkout rbb-arkhe. Ambos são experimentais; o segundo é exclusivamente fixture. O manifesto de execução fixa revisão Git, SHA-256 dos fontes, compilador e opções, ABI e hash do bytecode implantado. Configuração: solc 0.8.30, otimizador 200, viaIR, EVM Shanghai e chain ID 1337.

Interface de publicação existente:

```solidity
function record(Header calldata h, bytes calldata ed, bytes calldata ml)
    external returns (bytes32 id);
event Recorded(bytes32 indexed orgId, bytes32 indexed streamId,
    uint64 indexed sequence, bytes32 checkpointId,
    bytes32 commitment, bytes32 root, uint64 count);
```

O ABI compilado define a tupla `Header`, com os 27 campos na ordem de `header.rs`. Não existem `anchorCheckpoint` ou `fabricRefToCheckpoint` nesta implementação. Conferir `records(streamKey(orgId, streamId), sequence)`, que retorna `exists`, `id`, `commitment`, `root`, `count`, `blockNumber`, e o evento emitido pelo endereço esperado. Exigir recibo com sucesso e compatibilidade entre cabeçalho, evento, armazenamento e envelope. O contrato não verifica as assinaturas híbridas; o verificador as verifica separadamente.

Modo offline inicial: `LOCAL_OPERATOR_ATTESTED`. Um atestado Ed25519 do observador sintético vincula identidade da rede, gênese, endereço e código do contrato, bloco/hash, recibo, evento e leitura de armazenamento a um checkpoint. Sua chave pública é fixada externamente. A conclusão é condicional à confiança nesse observador; não há prova autônoma de consenso, armazenamento Ethereum ou QBFT. Este atestado é um perfil de teste separado, não o quórum institucional do núcleo. Verificações online no executor devem conferir as leituras reais antes de emitir o atestado.

### 2. Mapeamento do protocolo existente

Usar `arkhe-rbb-core::rbb::Header` e `Envelope`, protocolo v3, candidato revisão 2. Não usar `UnifiedCheckpointEnvelope` v2 nem alegar compatibilidade com um WASM v2. Cabeçalho: 864 bytes; envelope ABI: 3.488 bytes. Digest de assinatura: Keccak256(Keccak256(`AKG:CHECKPOINT:HEADER:v3` seguido de byte zero) || cabeçalho ABI). Contexto ML-DSA: `ARKHE_RBB_EVIDENCE_V3`.

| Campos | Regra |
|---|---|
| protocol_version / profile_id | 3 / 1 |
| anchor_chain_id | 1337, palavra uint256 ABI |
| anchor_genesis_hash | Hash real do bloco zero da execução |
| registry_address / anchor_address | Endereços reais implantados e fixados no atestado |
| org_id | Keccak256 dos bytes UTF-8 de `ARKHE:CUSTODY:PILOT:ORG:1` |
| power | Copiar o valor da fixture `arkhe/fixtures/protocol-v3.json`, validado no registry sintético; exclusivamente harness institucional de teste |
| stream_id | Keccak256 dos bytes UTF-8 de `ARKHE:CUSTODY:PILOT:STREAM:1` |
| sequence | 1 no pacote isolado; sequência crescente em testes de continuidade |
| prev_checkpoint_id / prev_present | Zero/false na primeira publicação; ID anterior/true nas demais |
| first_leaf_index / batch_leaf_count / total_leaf_count | Índice inicial, quantidade positiva de folhas do lote e soma; coerentes com o histórico acumulado |
| mmr_root / mmr_profile_digest | Calculados conforme perfil MMR v3; perfil fixado na configuração externa |
| batch_manifest_digest | Keccak256 do manifesto canônico do lote, sem campos circulares |
| policy_digest | Keccak256 do documento canônico de políticas e identificação das regras |
| declared_signed_at | Timestamp declarado determinístico do cenário; não constitui tempo confiável |
| source_kind | 4, outro aprovado pelo administrador sintético do stream |
| source_network_id | Keccak256 de `ARKHE:CUSTODY:PILOT:EVM:1337:` concatenado ao hash de gênese em hexadecimal minúsculo sem prefixo |
| source_manifest_digest / source_manifest_present | Keccak256 do manifesto canônico de fontes / true |
| ed25519_key_id / mldsa_key_id | Keccak256 das respectivas chaves públicas, conforme fixture existente |
| keyset_version | 1 |

Um banco não é classificado como Poder Executivo. O campo `power` é uma limitação do protocolo institucional reaproveitado no harness. Um perfil de organizações privadas exige desenho e versionamento próprios antes de uso real; não redefinir silenciosamente o enum. O piloto não testa Fabric; o caso `source_kind == 2` pertence à suíte existente.

### 3. Modelo de ameaças

| Ameaça | Cobertura |
|---|---|
| Omissão em uma fonte | Detectável quando uma fonte coberta fornece observação confrontável |
| Alteração após compromisso | Detectável por hash/assinaturas e vinculação ao checkpoint |
| Duplicidade, conflito e política trocada | Detectáveis pelas regras e allowlist externa |
| Colusão ou mesma operação omitida de todas as fontes | Não detectável; limitação obrigatória |
| Roubo de chave válida | Não resolvido; chaves são públicas e sintéticas no piloto |
| Substituição de bloco | Detectável quando observada pelo executor; offline depende do atestado e das informações exportadas |
| Verificador ou máquina comprometidos | Fora do escopo; revisão de código e execução independente são pressupostos |
| Pacote hostil | Parser estrito, limites, rejeição de caminhos inseguros e proibição de executar código do pacote |

### 4. Confirmação e substituição de blocos

Definir `depth = head.number - event.block_number`; evento elegível quando depth >= 3 e bloco pertence à cadeia canônica observada. Profundidade é convenção do teste, não garantia econômica ou finalização de uma rede real.

Antes de fechar o pacote, reler todos os hashes de blocos que sustentam saldos, eventos e âncora. Qualquer mudança gera `REFERENCE_BLOCK_CONFLICT`; a regra afetada fica não avaliável e a cobertura correspondente insuficiente, preservando divergências independentes. Remover eventos órfãos apenas em nova execução, com novo relatório e checkpoint; não editar um pacote comprometido. Não há janela após a qual uma substituição observada seja ignorada.

O cenário usa snapshot/revert da EVM local e mineração de ramo alternativo com transação distinta, garantindo hash diferente. O executor conserva ambas as observações. O verificador offline não descobre reorganizações posteriores que não foram fornecidas.

### 5. Compromisso e reprodução do relatório

Usar os dois mecanismos em testes separados. O manifesto do lote contém caminhos e hashes de entradas, política, regras e relatório. O cabeçalho compromete esse manifesto. O próprio manifesto não inclui seu hash, envelope, atestado ou recibo: evita-se ciclo.

Teste A altera apenas o relatório após publicação: exigir `REPORT_DIGEST_MISMATCH` antes de reexecução confiável. Teste B fornece um relatório incorreto porém comprometido e assinado pelo executor sintético: a reexecução deve gerar `REPORT_REPLAY_MISMATCH`. Troca de regras usa teste C: digest fora da allowlist gera `RULESET_NOT_ALLOWED`; nenhuma regra do pacote é executada.

Codificação do piloto: JSON UTF-8 sem BOM, chaves ASCII ordenadas lexicograficamente em todos os objetos, sem chaves duplicadas, sem espaços externos ou newline final; textos restritos a ASCII imprimível, aspas e barra invertida escapadas, sem escapes alternativos. Inteiros de domínio são strings decimais sem sinal positivo ou zeros à esquerda; negativos somente em lançamentos que permitam sinal, sem `-0`. Booleanos e null usam literais JSON. Proibir números JSON e campos desconhecidos. Arrays seguem ordem definida por schema: registros por identificador estável, arquivos por caminho ASCII, exceções por código e identificador. Schemas e vetores byte a byte devem ser publicados antes de aceitar pacotes.

Hash de arquivos: Keccak256 dos bytes canônicos. Perfil MMR continua sendo o existente, com seus domínios BLAKE3; não substituir um algoritmo pelo outro. Cada folha do lote contém uma entrada canônica do manifesto (caminho e digest); o manifesto completo contém as mesmas entradas, versão e ordem. Exigir correspondência e provas de inclusão/extensão pertinentes.

### 6. Tempo

Unix epoch UTC em segundos inteiros. Vigências são intervalos semiabertos `[valid_from, valid_until)`, compatíveis com a validação de keysets existente. Tolerância de skew: zero no cenário determinístico. Nunca inferir confiança apenas de `declared_signed_at`.

Usar timestamp do bloco de execução para avaliar vigência da política e da autorização; exigir autorização emitida antes ou no mesmo instante desse bloco. Vincular sua identidade à transação. Limites de valor são avaliados separadamente de vigência. O timestamp da assinatura não substitui o instante de execução.

Definir `T0 = 1803772800` como época fixa de teste e construir blocos e documentos com deslocamentos declarados no cenário. O executor deve falhar se o backend não respeitar os timestamps configurados. Testar exatamente início de vigência, um segundo antes do fim e o próprio fim.

### 7. Volume, limites e medição

Semente de dados fixa: 20261029. Benchmark sintético, por fonte: 1.000, 100.000 e 1.000.000 de registros de movimentação; registrar também total de registros, bytes, contas e exceções. Não alegar que todos esses registros foram executados na EVM: o teste de integração local usa 100 transferências reais; os volumes maiores exercitam geração e replay de snapshots sintéticos.

O nível de 1.000 registros e integração de 100 transferências são obrigatórios; os demais são níveis de capacidade que podem terminar `RESOURCE_LIMIT`, nunca aprovação silenciosa. Limites: pacote total 2 GiB, arquivo 512 MiB, até 128 arquivos e 1.000.000 de registros por fonte, JSON depth 16 e strings até 4.096 bytes. Rejeitar symlinks, caminhos absolutos, `..` e colisões de caminhos; nenhum arquivo de pacote pode definir confiança local.

Medir separadamente normalização, conciliação, empacotamento/assinatura, publicação EVM e replay offline; tempo monotônico, pico de memória residente, tamanho do pacote e taxa de registros. Registrar CPU/modelo/cores, RAM, SO, disco e versões das ferramentas. Uma execução de aquecimento e três medições por nível, com todas as amostras e mediana. Não há SLO comercial definido; timeout de execução de 300 segundos por nível produz resultado incompleto explícito. Comparar desempenho apenas com ambiente e conjunto equivalentes.

### 8. Governança de regras

No piloto, o mantenedor do repositório aprova o schema e o ruleset por revisão versionada. O verificador recebe uma configuração externa com IDs e digests permitidos. O pacote identifica essas versões, mas não pode ampliá-las. Não atribuir aprovação ao BCB, à RBB ou a um comitê inexistente. Governança de produção permanece pendente.

### 9. Chaves e reprodutibilidade

Reutilizar `rbb_sign_fixture`: Ed25519 e ML-DSA-44 derivados da seed pública de 32 bytes de valor 7; assinatura ML-DSA com seed de 32 bytes de valor 8 e contexto v3 existente. Essas chaves não oferecem autenticidade contra adversários que conhecem o fixture; apenas exercitam a verificação criptográfica.

Fontes e observador usam chaves Ed25519 distintas, derivadas respectivamente de arrays de 32 bytes de valores 11, 12, 13 e 14. Assinam bytes canônicos com prefixos ASCII de domínio exclusivos e byte zero: `ARKHE:CUSTODY:SOURCE:v1` e `ARKHE:CUSTODY:ANCHOR:v1`. O documento assinado identifica fonte, perímetro, digests e versão. Vetores de chaves públicas e assinaturas devem ser exportados como fixture externa revisada; o verificador compara com essa configuração e não confia na cópia incluída no pacote.

Chaves públicas de teste não constituem cerimônia institucional. Revogação operacional, HSM, rotação e recuperação permanecem fora do piloto. Pacotes com os mesmos dados e mesmas chaves devem ter conteúdo canônico e resultados iguais; execução EVM independente pode alterar gênese, endereços, blocos e hashes, portanto não se exige identidade binária entre implantações distintas. Replay do mesmo pacote deve produzir relatório idêntico byte a byte.
