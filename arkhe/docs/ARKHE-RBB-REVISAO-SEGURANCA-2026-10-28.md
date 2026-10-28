# Arkhe–RBB — revisão de arquitetura e segurança

Data: 28/10/2026. Estado: revisão estática do material proposto; não constitui certificação de implementação ou aprovação institucional.

## 1. Parecer

A separação entre sistemas de origem, evidências assinadas, acumulador MMR e ancoragem em Besu é aproveitável. O material, entretanto, não sustenta as garantias de verificação independente, integridade ponta a ponta, recuperação de falhas e conformidade que anuncia. Os exemplos não devem ser promovidos a produção sem corrigir os bloqueadores abaixo. Foram registrados 17 achados: 1 crítico, 13 altos e 3 médios, classificados pelo impacto nas garantias anunciadas.

A entrega complementar [Especificação técnica consolidada](ARKHE-RBB-ESPECIFICACAO-TECNICA-v0.1.md) transforma os achados em requisitos. Ela propõe uma nova versão de protocolo; não declara compatibilidade binária com os exemplos v2.

Evolução posterior de desenho: [especificação v0.2](ARKHE-RBB-ESPECIFICACAO-TECNICA-v0.2.md) e [arquitetura v0.2](ARKHE-RBB-ARQUITETURA-v0.2.md). Esses documentos propõem tratamentos para os achados; não representam correções já implementadas ou achados encerrados.

## 2. Escopo e método

Fonte: `Texto colado.txt`, anexo do usuário, com 6.123 linhas no inventário automatizado. As referências `Lx–Ly` abaixo são linhas desse anexo, e não arquivos existentes em `D:\arkhe`. Há versões repetidas do registro e propostas alternativas de envelope. Foram examinados os contratos, verificador Rust/Wasm, suíte Foundry, hooks e worker Django, ponte Fabric, gate e as afirmações arquiteturais associadas. A primeira ocorrência completa do registro é usada como referência.

O [inventário da revisão](ARKHE-RBB-REVISAO-INVENTARIO-2026-10-28.json) registra SHA-256 do anexo, localização e hash dos 38 blocos cercados por fences. O registro Solidity de L211–757 é idêntico, após normalização de finais de linha, ao de L2940–3486. O inventário permite identificar exatamente o material revisto; não valida os programas.

Método: inspeção de fluxo de dados e autorização, comparação dos formatos, análise de cenários de falha e consulta a fontes primárias. Não foram compilados os projetos colados, executadas as suítes Foundry/Rust, implantados contratos ou acessadas redes RBB/Fabric. O anexo não é um projeto autocontido: contém funções e módulos ausentes. As reproduções descritas são argumentos derivados dos caminhos do código, não explorações executadas em rede.

Severidade: crítica quando destrói a raiz de confiança anunciada; alta quando permite alteração de efeito autorizado, perda, duplicação ou falsa certificação; média para indisponibilidade, incompatibilidade e lacunas operacionais. Uma falha condicional ao comprometimento de operador é explicitamente indicada.

## 3. Achados

### A01 — Crítica: o pacote autentica suas próprias chaves

**Local:** `OfflineVerificationBundle.registry_keys`, `verify_offline_bundle`, `find_valid_key_in_registry`; anexo L943–1220.

O bundle contém envelope, chaves e seu histórico declarados pelo mesmo fornecedor. Recomputar `keccak256(pubkey)` só verifica consistência interna. Não prova que a chave pertence a `org_id`, que existiu no contrato, nem que estava autorizada no bloco relevante. O snapshot nem transporta uma prova autenticada de associação órgão–chave. Também não há validação de transação/recibo de ancoragem ou finalidade da RBB.

**Cenário:** um fornecedor malicioso gera seu próprio par Ed25519/ML-DSA, inventa um snapshot temporal, escolhe o órgão vítima, constrói MMR e assina um envelope. As verificações implementadas podem aprovar esse pacote sem qualquer registro ou transação real na RBB. Não é necessário quebrar a criptografia.

**Correção:** raiz de confiança externa ao bundle, identidade de rede e contratos fixadas, prova de estado do registro e prova de ancoragem ligadas a cabeçalho autenticado. Alternativamente, atestação independente explicitamente confiada, com resultado denominado “atestado”, sem anunciar verificação autônoma do consenso. Sem essa evidência, retornar `INDETERMINATE_TRUST`.

### A02 — Alta: a ancoragem registra declarações sem vinculá-las ao envelope

**Local:** `ArkheCheckpointAnchor.anchorCheckpoint`; L5703–5795.

O contrato verifica o operador ativo, mas não decodifica o envelope, não compara seus campos aos parâmetros e não verifica assinaturas. `envelopeCborCalldata` pode ser vazio. O `verifyMldsaCheckpointOnChain` do registro não é chamado pelo anchor. `power`, `sourceChain`, `signedAt` e nível criptográfico são escolhidos pelo operador, sem conferir poder cadastrado, enum, intervalo temporal ou chaves.

**Cenário:** operador autorizado, comprometido ou com bug registra um root arbitrário com envelope ausente ou divergente. A transação continua sendo uma declaração autenticada da conta EVM, mas não uma evidência híbrida validada.

**Correção:** distinguir `RECORDED` de `VERIFIED`. No perfil inicial, comprometer os bytes integrais por hash verificável em EVM e exigir sua igualdade no verificador independente. Se houver perfil de verificação criptográfica no contrato, ele deve validar o mesmo payload e política antes de marcar `VERIFIED`. A disponibilidade de calldata não substitui essa ligação.

### A03 — Alta: uma assinatura de registro pode autorizar uma revogação por rotação

**Local:** `REGISTER_TYPEHASH`, `registerKey`, `rotateKey`; L278–282 e L438–576.

As duas funções assinam o mesmo tipo `RegisterKey`. A rotação inclui o hash da chave antiga no campo `migrationFrom`, mas não um tipo de operação distinto. O registro normal também aceita esse campo. Se os campos, o nonce e o tempo efetivo coincidirem, a mesma autorização serve às duas chamadas com efeitos diferentes.

**Cenário:** o operador assina a adição de uma chave com `migrationFrom = hashAntiga` e `validFrom = T`. Um relayer apresenta a autorização em `rotateKey` num bloco com timestamp T. A chave antiga é revogada, embora a mensagem assinada não autorize explicitamente esse efeito. O nonce evita o segundo uso; não evita escolher a operação errada no primeiro uso.

**Correção:** tipos EIP-712 separados para `RegisterKey`, `RotateKey` e `RevokeKey`; rotação deve incluir índice antigo, nova chave, tempo explícito, nonce, época do operador e deadline. O teste deve mostrar que uma autorização de registro nunca serve à rotação.

### A04 — Média: assinatura de meta-transação depende de timestamp de inclusão desconhecido

**Local:** `effectiveValidFrom` em `registerKey` e `nowTs` no hash de `rotateKey`; L452–479 e L530–553.

Com `validFrom == 0`, o contrato substitui o valor por `block.timestamp` antes de validar a assinatura. Na rotação, esse timestamp nem é argumento. O signatário externo não controla o instante do bloco futuro. Os testes usam `vm.warp` e assinatura no mesmo tempo, mascarando a fragilidade operacional.

**Correção:** assinar parâmetros explícitos e estáveis; conferir janela de submissão separadamente. Nunca inserir no hash autorizado um timestamp que só será conhecido na inclusão.

### A05 — Alta: falhas de unicidade, ordem e isolamento no anchor

**Local:** mappings e `anchorCheckpoint`; L5711–5787.

`checkpoints[hash].signedAt != 0` é usado como sentinela, mas `signedAt = 0` é aceito: o mesmo slot pode ser reescrito. O hash do checkpoint é global e não inclui necessariamente órgão, stream ou rede. `prevCheckpointHash` não é comparado ao último checkpoint. `fabricRefToCheckpoint` permite sobrescrita e usa namespace global. Um operador de outro órgão pode ocupar identificadores quando sua composição permitir.

**Correção:** existência explícita; chave `(orgId, streamId, sequence)`; sequência monotônica, predecessor esperado, identificador de rede e replay idempotente apenas dos mesmos bytes. Referência Fabric deve incorporar rede/canal/tipo e admitir a semântica de cardinalidade necessária.

### A06 — Alta: a integração não corresponde à EIP-8051 consultada

**Local:** enum, `verifyMldsaCheckpointOnChain` e mock; L241–254, L652–692 e L1537–1588.

O contrato envia `abi.encode(algorithm, pubkey, messageHash, signature)`. O rascunho consultado descreve entrada de comprimento fixo, mensagem, assinatura e chave expandida; não essa ABI. Também não fundamenta o roteamento 44/65/87 apresentado. O mock apenas reproduz a interface inventada e confere uma tag calculável por qualquer pessoa. Ele testa integração local, não ML-DSA. A variante ETH não se torna verificável pela implementação FIPS apenas trocando o contexto de assinatura.

**Correção:** remover essa dependência do perfil inicial e rejeitar algoritmos ETH nesse perfil. Tratar qualquer precompile como experimento dependente de versão do cliente, revisão da proposta, codec e testes reais. Fonte: [EIP-8051](https://eips.ethereum.org/EIPS/eip-8051). O status Draft e o documento público não provam disponibilidade em Besu ou na RBB. Os 4.500 gas mencionados são valor da proposta, não benchmark da aplicação.

### A07 — Alta: `signed_at` permite alegar anterioridade após comprometimento

**Local:** `_isKeyValidAt`, consulta histórica, snapshot e `signed_at` escolhido pelo emissor; L615–720, L1138–1188 e L5580–5610.

Uma chave revogada pode assinar depois da revogação declarando data anterior. Verificar a validade na data autodeclarada não prova quando a assinatura foi criada. Registro de chave com `validFrom` passado também não prova que a autorização existia naquele passado.

**Correção:** distinguir tempo declarado, tempo de registro e primeira ancoragem autenticada. Preservar validade histórica somente com evidência de existência anterior ao marco de comprometimento, ou política explícita de incerteza. Registrar motivo e limite de comprometimento, sem reescrever checkpoints anteriores.

### A08 — Alta: a prova MMR não valida a geometria da árvore

**Local:** `MmrInclusionProof` e `verify_mmr_inclusion`; L931–942 e L1328–1389.

`leaf_count` só é usado para testar `index < count`. Não se verifica se a quantidade de siblings corresponde à altura da montanha, nem se o número e a posição dos picos correspondem à decomposição binária de `leaf_count`. O envelope não assina esse contador. O cálculo deve usar índice local à montanha; nos tamanhos válidos alinhados sua paridade coincide com a global, portanto a simples variável global não demonstra isoladamente um erro.

**Reprodução por inspeção:** uma prova válida de uma folha, sem siblings nem picos, permanece aceita se `leaf_count` for alterado de 1 para 2. A função não passa a exigir o sibling que essa nova árvore requer. Isso falsifica o tamanho declarado; não demonstra inclusão arbitrária contra uma raiz honesta sob resistência a colisões.

**Correção:** assinar `leaf_count`; derivar montanhas e pico-alvo do contador e índice; exigir exatamente a altura e picos esperados, validar limites e terminar o cálculo com contador vinculado à raiz.

### A09 — Alta: CBOR e schemas não têm um contrato comum demonstrado

**Local:** `cbor2.dumps(canonical=True)`, `ciborium::into_writer`, enums Serde e hash do grafo; L2327–2360, L5475–5485 e L5910–5960.

Serializar uma struct Rust não demonstra igualdade com o mapa Python. `#[repr(u8)]` não instrui Serde a codificar o enum como inteiro; as structs da ponte usam `u8`, enquanto o verificador usa enums. Arrays `[u8; 32]` e `Vec<u8>` também exigem codec explícito para virar CBOR byte strings, em vez de arrays de inteiros. O verificador apenas hasheia `wer_cbor_bytes`, sem validar schema/canonicalidade, e aceita domínio fornecido pelo pacote.

O digest do `ConstitutionGraph` depende de `HashMap` em JSON, sem ordenação canônica e com `unwrap_or_default()` que esconde erros. Um algoritmo determinístico de hash não corrige entrada não determinística.

**Correção:** um schema versionado por tipo, codec binário explícito, política única de determinismo, rejeição de campos/tipos desconhecidos e vetores idênticos Rust/Python. Hash do grafo sobre regras ordenadas e versão do avaliador. Referência: [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html).

### A10 — Alta: hooks não asseguram outbox atômico e completo

**Local:** `signals.py`; L2418–2565.

`post_save` não torna a gravação do ato e da outbox automaticamente atômicas em todos os caminhos. Exceções são suprimidas e substituições por hashes zero escondem captura incompleta. O callback `on_commit` pode falhar depois de sair do `try` que o registrou. Operações de atualização em lote podem contornar sinais. O estado consultado em `pre_save`/`post_save` pode não corresponder à transição de negócio completa. A intenção é escolhida pela última matéria, sem vínculo explícito ao evento.

Há uma decisão real de produto: se o banco não consegue persistir a evidência, não é possível prometer simultaneamente ato sempre disponível e captura síncrona sem perdas. `on_commit` só agenda após commit; não é fila durável. Fonte: [transações Django](https://docs.djangoproject.com/en/5.2/topics/db/transactions/).

**Correção:** ato e outbox no serviço transacional de negócio, dispatcher que consulta pendências, reconciliação por IDs/revisões e estado `EVIDENCE_GAP` para captura ausente. Política explícita para falha local, separada de indisponibilidade de broker ou RBB.

### A11 — Alta: workers não são idempotentes e certificam envio como confirmação

**Local:** `process_outbox_event_task`, `build_and_anchor_checkpoint_task`; L2594–2677.

Dois workers podem ler PENDING e inserir a mesma folha. Uma queda após `append_leaf`, antes do save, duplica a inserção no retry. Não há transação comum do MMR com a outbox, nem lock por órgão. O fechamento seleciona só eventos já processados e pode assinar uma sessão parcial; a raiz é a do MMR global, enquanto `checkpoint_hash` agrega atos selecionados da sessão sem provar a relação entre esses conjuntos. `prev_checkpoint_hash` é sempre ausente. Receber `tx_hash` leva diretamente a `ANCHORED_RBB`, sem receipt com sucesso nem finalidade. `max_retries` sozinho não provoca retry de exceções.

**Correção:** chave de idempotência por evento de origem, exclusão mútua por stream, append e progresso na mesma transação, barreira de fechamento, máquina de estados durável e confirmação do conteúdo do recibo/bloco. Hash de payload não substitui ID do evento: ocorrências legítimas iguais podem existir.

### A12 — Alta: ponte não implementa a recuperação e a autenticidade anunciadas

**Local:** `FabricToRbbBridge`, `ingest_fabric_event`, `seal_batch_checkpoint`; L5356–5695.

O exemplo recebe uma struct em memória com `validation_code`; não implementa gRPC, autenticação do peer, prova de origem, persistência ou submissão. `channel_id` e `chaincode_name` recebidos não são conferidos contra allowlist. O sealing limpa buffers e avança o predecessor antes de confirmação da RBB. Uma queda pode perder o lote.

Retomar no bloco seguinte só é seguro depois de persistir todo o bloco; uma queda no meio exige cursor por transação/evento e deduplicação. `DeliverFiltered` omite payload de eventos, portanto não fornece sozinho os digests de estado esperados. Fonte: [serviços de eventos Fabric](https://hyperledger-fabric.readthedocs.io/en/latest/peer_event_services.html).

**Correção:** modo explícito de confiança no peer/ponte; usar serviço que exponha payload autorizado ou busca complementar autenticada; persistir origem, ordem, evento, WER, MMR e lote antes do ACK. Separar lote selado, submetido e confirmado. Não anunciar light client Fabric nem ponte sem confiança.

### A13 — Alta: gate aprova entradas cuja política não foi verificada

**Local:** `ConstitutionGraph::compute_digest`, `ContainmentGate::evaluate`; L5910–6054.

Tipo de ato sem regras retorna `Ok(())` antes dos checks universais. Regras desconhecidas são ignoradas. A versão e o digest do grafo não são conferidos. O emissor fornece seu próprio `required_quorum`. `round_number != 0` aceita turno 3 ou 255 e não comprova dois turnos. Uma lista vazia de pré-condições passa. A soma de timestamps pode estourar e não há rejeição de emissão futura. O worker SAPL mostrado nem chama o gate para avaliar intenções.

**Correção:** política e quórum carregados de configuração autenticada; desconhecido/incompleto deve resultar em `INDETERMINATE`, sem selo de conformidade. Validar todos os limites antes de buscar regras. Pré-condições devem ser provas tipadas verificadas, e não booleano fornecido por chamador não autenticado. Regras jurídicas ilustrativas do texto dependem de validação institucional; o código não as torna aplicáveis a toda casa legislativa.

### A14 — Alta: governança e autorização não têm proteção PQ ponta a ponta

**Local:** `superAdmin`, `setCommitteeAdmin`, `setPrecompileAddresses`, `_verifyOperatorAuth`; L286–433 e L725–755.

Um administrador pode trocar o verificador para um contrato que sempre retorna true. O operador pode substituir chaves sem prova de posse nem aprovação PQ independente. ECDSA e contas administrativas continuam sendo raízes de autorização clássicas. A assinatura híbrida da evidência não torna consenso, registro e recuperação de chaves resistentes a um adversário quântico. Rotacionar Ed25519 para ML-DSA revogando a única chave Ed25519 também interrompe o perfil híbrido AND.

**Correção:** separar papéis e famílias de chaves, rotação dentro da família e ativação de keysets completos, administração por quórum com trilha auditável, mudança de verificador controlada e recuperação ensaiada. Caracterizar proteção PQ como propriedade do payload com dependências clássicas explicitadas.

### A15 — Média: completude de implementação e testes superestimada

**Local:** manifesto/teste Rust L776–807 e L1395–1478; helpers SAPL e ponte; suíte Foundry L1529–2093.

Não há pacote completo que permita reproduzir os resultados anunciados. `_submit_envelope_to_rbb`, `WormGraphMMR`, serviços de listener/persistência e outros componentes não estão implementados no material. O RNG e as APIs/traits precisam ser conferidos contra versões fixadas das bibliotecas. O mock usa chaves aleatórias do tamanho certo e assinatura simulada; isso não prova parsing, posse nem conformidade FIPS. A quantidade “37 golden vectors” não vem acompanhada do conjunto completo e dos resultados reproduzíveis no anexo.

**Correção:** tratar API/build como pendentes até compilação, fixar lockfiles e toolchain, usar vetores oficiais e testes diferenciais. Testes de autorização com mocks são úteis, mas devem ser rotulados conforme seu alcance. Nenhum número de cobertura ou benchmark foi confirmado nesta revisão.

### A16 — Alta: hash não assegura anonimização ou conformidade automática

**Local:** tabela LGPD L5800–5807 e descrições CIN/SAPL.

Hashes de identificadores ou de estados previsíveis podem permitir busca por dicionário e correlação com outros registros. Metadados como órgão, UF, horário e referência também podem revelar informação. O digest não destrói o dado original nem comprova, por si, anonimização. A ANPD trata a avaliação de reidentificação como dependente do contexto no [estudo técnico sobre anonimização](https://www.gov.br/anpd/pt-br/centrais-de-conteudo/documentos-tecnicos-orientativos/estudo_tecnico_sobre_anonimizacao_de_dados_na_lgpd___analise_juridica.pdf/@@download/file).

**Correção:** minimização, avaliação de necessidade e risco, retenção e controle de acesso fora da cadeia; compromissos com aleatoriedade secreta quando apropriado e política de revelação. A conformidade depende do tratamento e da governança, não só da primitiva criptográfica. A revisão não certifica adequação jurídica.

### A17 — Média: recursos e disponibilidade sem limites verificáveis

**Local:** vetores dinâmicos do bundle, consulta `getValidKeysAt`, envelope calldata e inferências “gas-free”.

JSON, WER e provas não têm limites explícitos. `getValidKeysAt` percorre e copia todo o histórico com chaves grandes, podendo exceder limites de resposta/RPC. Precompile configurável sem controle de gas/retorno cria risco de indisponibilidade. Preço de gas zero não elimina gas por bloco, custo de armazenamento ou exaustão de CPU. Versionar um Wasm e seu SHA no mesmo local não impede substituição conjunta.

**Correção:** limites de tamanho/profundidade/trabalho, paginação, limites de chamada, política de publicação autenticada e repositórios de evidência redundantes. Assinatura Sigstore deve verificar identidade e issuer esperados; não prova que o código é correto nem que o build é reproduzível. Fonte: [verificação Sigstore](https://docs.sigstore.dev/cosign/verifying/verify/).

## 4. Correções de arquitetura e de narrativa

1. Manter módulos de aplicação separados da configuração de rede. Não incluir genesis de produção nem presumir autorização para novo validador. A página pública do [RBBNet/rbb](https://github.com/RBBNet/rbb) sustenta seu foco em infraestrutura; uma árvore superficial não prova ausência de código em todo histórico. `util/` e `bndes-network/` da proposta não foram confirmados na listagem atual consultada.
2. Chamar a ponte de ponte de evidências com confiança explícita no emissor. Ela não verifica automaticamente o consenso Fabric na EVM nem executa transferência de ativos.
3. Separar fonte do ato, destino da âncora e serviço de tempo. SAPL não se torna “origem Besu” só porque publica na RBB; TSA é serviço de timestamp, não necessariamente blockchain.
4. Distinguir integridade dos bytes, assinatura autorizada, inclusão no MMR, ancoragem e conformidade de política. Nenhuma dessas propriedades isolada demonstra veracidade material ou completude dos atos de origem.
5. Manter consenso e transporte inalterados no piloto. Adicionar precompile ou substituir assinaturas de bloco é mudança de cliente/rede, fora da promessa de extensão não invasiva.
6. Usar marcos por evidência e dependências. Datas, instituições parceiras, ACTs, aprovação de schema, hardware e topologia apresentados no anexo permanecem pendentes de documentação atual. O identificador RFC-RBB-2026-04 é rótulo da proposta, não prova de registro oficial.

## 5. Priorização e validação necessária

| Ordem | Trabalho | Achados | Evidência de encerramento |
|---|---|---|---|
| P0 | Raiz de confiança e estados de verificação | A01, A02, A07 | Bundle forjado rejeitado; ancoragem provada; data declarada não contorna revogação |
| P0 | Autorização e identidade | A03–A05, A14 | Testes entre funções/redes/órgãos; rotação segura; idempotência do anchor |
| P0 | Codificação e MMR | A08, A09 | Vetores iguais em duas linguagens; provas malformadas rejeitadas |
| P0 | Captura e recuperação | A10–A12 | Fault injection em cada transição; nenhuma perda silenciosa; reconciliação completa |
| P0 | Política e privacidade | A13, A16 | Casos normativos aprovados; entradas incompletas indeterminadas; avaliação de exposição |
| P1 | Build, publicação e capacidade | A06, A15, A17 | Builds reproduzidos; testes reais; limites de carga e recuperação medidos |

As prioridades organizam correção, não substituem severidade. Um piloto com dados públicos e rede isolada pode ajudar a validar o protocolo; produção depende de fechar os bloqueadores e documentar riscos aceitos.

## 6. Registro de certeza

| Afirmação | Classificação nesta revisão |
|---|---|
| A01–A05, A07–A14 e A17 derivam dos caminhos descritos | Confirmado por inspeção estática; exploração não executada |
| A ABI apresentada diverge da EIP-8051 consultada | Confirmado por comparação documental |
| ML-DSA é padronizado em FIPS 204 | Confirmado pelo [NIST](https://csrc.nist.gov/pubs/fips/204/final) |
| Bibliotecas e exemplos compilam juntos | Pendente de projeto completo e build |
| RBB dispõe das precompiles e aceita este deploy | Não demonstrado |
| Hashes da CIN/SAPL garantem anonimização | Não demonstrado; garantia geral rejeitada |
| Medidas propostas resolvem os achados | Requisitos de correção; pendentes de implementação e testes |

Não se usa “LEMMA” como sinônimo de alegação ou observação parcial. A revisão não demonstra teoremas de segurança.
