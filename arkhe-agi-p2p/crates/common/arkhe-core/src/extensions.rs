use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use thiserror::Error;

/// As 9 camadas lógicas da especificação normativa de extensões Arkhe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArkheLayer {
    /// 1. Identidade e Conteúdo
    IdentityAndContent,
    /// 2. Manifestos e Modelos
    ManifestsAndModels,
    /// 3. Recibos e Evidência
    ReceiptsAndEvidence,
    /// 4. Rede e Descoberta
    NetworkAndDiscovery,
    /// 5. Inferência e Execução
    InferenceAndExecution,
    /// 6. Verificação e Fraudes
    VerificationAndFraud,
    /// 7. Economia e Governança
    EconomyAndGovernance,
    /// 8. Integração com Chains (CAL)
    ChainIntegration,
    /// 9. Build, Testes e Supply Chain
    BuildAndSupplyChain,
}

/// Nível de obrigatoriedade de arquivo conforme a especificação
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExtensionRequirement {
    /// Obrigatória no protocolo ou subsistema correspondente
    Mandatory,
    /// Recomendada
    Recommended,
    /// Opcional
    Optional,
    /// Restrita (sujeita a condições rígidas de integridade / nunca produção não atestada)
    Restricted,
    /// Proibida no protocolo / produção
    Prohibited,
}

/// Extensão normativa reconhecida ou classificada pelo Arkhe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArkheExtension {
    // 1. Identidade e conteúdo
    Cid,
    Multihash,
    Pub,
    Sig,
    Cert,
    Crl,
    Keystore,
    Seed,

    // 2. Manifestos e modelos
    Arkmanifest,
    ArkmanifestSig,
    Gguf,
    GgufPart,
    GgufIndex,
    Quant,
    Lineage,
    Eval,
    Dataset,
    Tokenizer,
    Vocab,

    // 3. Recibos e evidência
    Afir,
    AfirJson,
    Afirtree,
    Wer,
    Mmr,
    Mmrproof,
    Consistency,
    Checkpoint,
    Attest,
    Arkbundle,
    Wasm,

    // 4. Rede e descoberta
    Peers,
    Peerledger,
    Seeds,
    Dhtcache,
    Announce,
    Gossip,
    Netpolicy,
    Pcap,

    // 5. Inferência e execução
    Policy,
    PolicySig,
    Request,
    Response,
    Session,
    Router,
    Expertmap,
    Kv,
    Gpu,
    Onnx,
    Safetensors,

    // 6. Verificação e fraudes
    Fraudproof,
    Reexec,
    Zkproof,
    Teequote,
    Verdict,
    Slash,
    Golden,

    // 7. Economia e governança
    Constitution,
    ConstitutionSig,
    Proposal,
    Vote,
    Canon,
    Emission,
    Reward,
    Stake,
    Epoch,
    Gini,

    // 8. Integração com chains
    Anchor,
    Ibc,
    Ccip,
    Layerzero,
    Abi,
    Sol,
    Move,
    Tx,
    Receiptroot,

    // 9. Build, testes e supply chain
    Toml,
    Rs,
    Ts,
    Cs,
    Csproj,
    Wat,
    Json,
    Lock,
    Sbom,
    Provenance,
    Sha256,
    Sigstore,
    Log,
    Trace,

    // Restritas ou Proibidas
    Dll,
    Exe,
    So,
    Dylib,
    Pem,
    Env,
    Bak,
    Tmp,
    Dump,
}

impl ArkheExtension {
    /// Retorna a representação textual canônica da extensão (com ponto inicial)
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cid => ".cid",
            Self::Multihash => ".multihash",
            Self::Pub => ".pub",
            Self::Sig => ".sig",
            Self::Cert => ".cert",
            Self::Crl => ".crl",
            Self::Keystore => ".keystore",
            Self::Seed => ".seed",

            Self::Arkmanifest => ".arkmanifest",
            Self::ArkmanifestSig => ".arkmanifest.sig",
            Self::Gguf => ".gguf",
            Self::GgufPart => ".gguf.part",
            Self::GgufIndex => ".gguf.index",
            Self::Quant => ".quant",
            Self::Lineage => ".lineage",
            Self::Eval => ".eval",
            Self::Dataset => ".dataset",
            Self::Tokenizer => ".tokenizer",
            Self::Vocab => ".vocab",

            Self::Afir => ".afir",
            Self::AfirJson => ".afir.json",
            Self::Afirtree => ".afirtree",
            Self::Wer => ".wer",
            Self::Mmr => ".mmr",
            Self::Mmrproof => ".mmrproof",
            Self::Consistency => ".consistency",
            Self::Checkpoint => ".checkpoint",
            Self::Attest => ".attest",
            Self::Arkbundle => ".arkbundle",
            Self::Wasm => ".wasm",

            Self::Peers => ".peers",
            Self::Peerledger => ".peerledger",
            Self::Seeds => ".seeds",
            Self::Dhtcache => ".dhtcache",
            Self::Announce => ".announce",
            Self::Gossip => ".gossip",
            Self::Netpolicy => ".netpolicy",
            Self::Pcap => ".pcap",

            Self::Policy => ".policy",
            Self::PolicySig => ".policy.sig",
            Self::Request => ".request",
            Self::Response => ".response",
            Self::Session => ".session",
            Self::Router => ".router",
            Self::Expertmap => ".expertmap",
            Self::Kv => ".kv",
            Self::Gpu => ".gpu",
            Self::Onnx => ".onnx",
            Self::Safetensors => ".safetensors",

            Self::Fraudproof => ".fraudproof",
            Self::Reexec => ".reexec",
            Self::Zkproof => ".zkproof",
            Self::Teequote => ".teequote",
            Self::Verdict => ".verdict",
            Self::Slash => ".slash",
            Self::Golden => ".golden",

            Self::Constitution => ".constitution",
            Self::ConstitutionSig => ".constitution.sig",
            Self::Proposal => ".proposal",
            Self::Vote => ".vote",
            Self::Canon => ".canon",
            Self::Emission => ".emission",
            Self::Reward => ".reward",
            Self::Stake => ".stake",
            Self::Epoch => ".epoch",
            Self::Gini => ".gini",

            Self::Anchor => ".anchor",
            Self::Ibc => ".ibc",
            Self::Ccip => ".ccip",
            Self::Layerzero => ".layerzero",
            Self::Abi => ".abi",
            Self::Sol => ".sol",
            Self::Move => ".move",
            Self::Tx => ".tx",
            Self::Receiptroot => ".receiptroot",

            Self::Toml => ".toml",
            Self::Rs => ".rs",
            Self::Ts => ".ts",
            Self::Cs => ".cs",
            Self::Csproj => ".csproj",
            Self::Wat => ".wat",
            Self::Json => ".json",
            Self::Lock => ".lock",
            Self::Sbom => ".sbom",
            Self::Provenance => ".provenance",
            Self::Sha256 => ".sha256",
            Self::Sigstore => ".sigstore",
            Self::Log => ".log",
            Self::Trace => ".trace",

            Self::Dll => ".dll",
            Self::Exe => ".exe",
            Self::So => ".so",
            Self::Dylib => ".dylib",
            Self::Pem => ".pem",
            Self::Env => ".env",
            Self::Bak => ".bak",
            Self::Tmp => ".tmp",
            Self::Dump => ".dump",
        }
    }

    /// Tenta identificar uma extensão Arkhe a partir do caminho ou nome de arquivo
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        let name = path.as_ref().file_name()?.to_str()?;
        Self::from_filename(name)
    }

    /// Tenta identificar uma extensão Arkhe a partir do nome do arquivo
    pub fn from_filename(name: &str) -> Option<Self> {
        let name_lower = name.to_lowercase();

        // Extensões compostas verificadas primeiro
        if name_lower.ends_with(".arkmanifest.sig") {
            return Some(Self::ArkmanifestSig);
        }
        if name_lower.ends_with(".afir.json") {
            return Some(Self::AfirJson);
        }
        if name_lower.ends_with(".policy.sig") {
            return Some(Self::PolicySig);
        }
        if name_lower.ends_with(".constitution.sig") {
            return Some(Self::ConstitutionSig);
        }
        if name_lower.ends_with(".gguf.index") {
            return Some(Self::GgufIndex);
        }
        if name_lower.contains(".gguf.part") {
            return Some(Self::GgufPart);
        }

        // Casos especiais de arquivos conhecidos por tabela
        if name_lower == "tokenizer.json" {
            return Some(Self::Tokenizer);
        }
        if name_lower == "vocab.txt" {
            return Some(Self::Vocab);
        }
        if name_lower == "peers.dat" {
            return Some(Self::Peers);
        }
        if name_lower == "kvcache.bin" {
            return Some(Self::Kv);
        }
        if name_lower == "gpu.profile" {
            return Some(Self::Gpu);
        }
        if name_lower == "emission.schedule" {
            return Some(Self::Emission);
        }
        if name_lower == "seeds.toml" {
            return Some(Self::Seeds);
        }
        if name_lower == "dht.cache" {
            return Some(Self::Dhtcache);
        }

        // Extensões de sufixo simples
        if name_lower.ends_with(".cid") {
            return Some(Self::Cid);
        }
        if name_lower.ends_with(".multihash") {
            return Some(Self::Multihash);
        }
        if name_lower.ends_with(".pub") {
            return Some(Self::Pub);
        }
        if name_lower.ends_with(".sig") {
            return Some(Self::Sig);
        }
        if name_lower.ends_with(".cert") {
            return Some(Self::Cert);
        }
        if name_lower.ends_with(".crl") {
            return Some(Self::Crl);
        }
        if name_lower.ends_with(".keystore") {
            return Some(Self::Keystore);
        }
        if name_lower.ends_with(".seed") {
            return Some(Self::Seed);
        }

        if name_lower.ends_with(".arkmanifest") {
            return Some(Self::Arkmanifest);
        }
        if name_lower.ends_with(".gguf") {
            return Some(Self::Gguf);
        }
        if name_lower.ends_with(".quant") {
            return Some(Self::Quant);
        }
        if name_lower.ends_with(".lineage") {
            return Some(Self::Lineage);
        }
        if name_lower.ends_with(".eval") {
            return Some(Self::Eval);
        }
        if name_lower.ends_with(".dataset") {
            return Some(Self::Dataset);
        }
        if name_lower.ends_with(".tokenizer") {
            return Some(Self::Tokenizer);
        }
        if name_lower.ends_with(".vocab") {
            return Some(Self::Vocab);
        }

        if name_lower.ends_with(".afir") {
            return Some(Self::Afir);
        }
        if name_lower.ends_with(".afirtree") {
            return Some(Self::Afirtree);
        }
        if name_lower.ends_with(".wer") {
            return Some(Self::Wer);
        }
        if name_lower.ends_with(".mmr") {
            return Some(Self::Mmr);
        }
        if name_lower.ends_with(".mmrproof") {
            return Some(Self::Mmrproof);
        }
        if name_lower.ends_with(".consistency") {
            return Some(Self::Consistency);
        }
        if name_lower.ends_with(".checkpoint") {
            return Some(Self::Checkpoint);
        }
        if name_lower.ends_with(".attest") {
            return Some(Self::Attest);
        }
        if name_lower.ends_with(".arkbundle") {
            return Some(Self::Arkbundle);
        }
        if name_lower.ends_with(".wasm") {
            return Some(Self::Wasm);
        }

        if name_lower.ends_with(".peers") {
            return Some(Self::Peers);
        }
        if name_lower.ends_with(".peerledger") {
            return Some(Self::Peerledger);
        }
        if name_lower.ends_with(".seeds") {
            return Some(Self::Seeds);
        }
        if name_lower.ends_with(".dhtcache") {
            return Some(Self::Dhtcache);
        }
        if name_lower.ends_with(".announce") {
            return Some(Self::Announce);
        }
        if name_lower.ends_with(".gossip") {
            return Some(Self::Gossip);
        }
        if name_lower.ends_with(".netpolicy") {
            return Some(Self::Netpolicy);
        }
        if name_lower.ends_with(".pcap") {
            return Some(Self::Pcap);
        }

        if name_lower.ends_with(".policy") {
            return Some(Self::Policy);
        }
        if name_lower.ends_with(".request") {
            return Some(Self::Request);
        }
        if name_lower.ends_with(".response") {
            return Some(Self::Response);
        }
        if name_lower.ends_with(".session") {
            return Some(Self::Session);
        }
        if name_lower.ends_with(".router") {
            return Some(Self::Router);
        }
        if name_lower.ends_with(".expertmap") {
            return Some(Self::Expertmap);
        }
        if name_lower.ends_with(".kv") {
            return Some(Self::Kv);
        }
        if name_lower.ends_with(".gpu") {
            return Some(Self::Gpu);
        }
        if name_lower.ends_with(".onnx") {
            return Some(Self::Onnx);
        }
        if name_lower.ends_with(".safetensors") {
            return Some(Self::Safetensors);
        }

        if name_lower.ends_with(".fraudproof") {
            return Some(Self::Fraudproof);
        }
        if name_lower.ends_with(".reexec") {
            return Some(Self::Reexec);
        }
        if name_lower.ends_with(".zkproof") {
            return Some(Self::Zkproof);
        }
        if name_lower.ends_with(".teequote") {
            return Some(Self::Teequote);
        }
        if name_lower.ends_with(".verdict") {
            return Some(Self::Verdict);
        }
        if name_lower.ends_with(".slash") {
            return Some(Self::Slash);
        }
        if name_lower.ends_with(".golden") {
            return Some(Self::Golden);
        }

        if name_lower.ends_with(".constitution") {
            return Some(Self::Constitution);
        }
        if name_lower.ends_with(".proposal") {
            return Some(Self::Proposal);
        }
        if name_lower.ends_with(".vote") {
            return Some(Self::Vote);
        }
        if name_lower.ends_with(".canon") {
            return Some(Self::Canon);
        }
        if name_lower.ends_with(".emission") {
            return Some(Self::Emission);
        }
        if name_lower.ends_with(".reward") {
            return Some(Self::Reward);
        }
        if name_lower.ends_with(".stake") {
            return Some(Self::Stake);
        }
        if name_lower.ends_with(".epoch") {
            return Some(Self::Epoch);
        }
        if name_lower.ends_with(".gini") {
            return Some(Self::Gini);
        }

        if name_lower.ends_with(".anchor") {
            return Some(Self::Anchor);
        }
        if name_lower.ends_with(".ibc") {
            return Some(Self::Ibc);
        }
        if name_lower.ends_with(".ccip") {
            return Some(Self::Ccip);
        }
        if name_lower.ends_with(".layerzero") {
            return Some(Self::Layerzero);
        }
        if name_lower.ends_with(".abi") {
            return Some(Self::Abi);
        }
        if name_lower.ends_with(".sol") {
            return Some(Self::Sol);
        }
        if name_lower.ends_with(".move") {
            return Some(Self::Move);
        }
        if name_lower.ends_with(".tx") {
            return Some(Self::Tx);
        }
        if name_lower.ends_with(".receiptroot") {
            return Some(Self::Receiptroot);
        }

        if name_lower.ends_with(".toml") {
            return Some(Self::Toml);
        }
        if name_lower.ends_with(".rs") {
            return Some(Self::Rs);
        }
        if name_lower.ends_with(".ts") {
            return Some(Self::Ts);
        }
        if name_lower.ends_with(".cs") {
            return Some(Self::Cs);
        }
        if name_lower.ends_with(".csproj") {
            return Some(Self::Csproj);
        }
        if name_lower.ends_with(".wat") {
            return Some(Self::Wat);
        }
        if name_lower.ends_with(".json") {
            return Some(Self::Json);
        }
        if name_lower.ends_with(".lock") {
            return Some(Self::Lock);
        }
        if name_lower.ends_with(".sbom") {
            return Some(Self::Sbom);
        }
        if name_lower.ends_with(".provenance") {
            return Some(Self::Provenance);
        }
        if name_lower.ends_with(".sha256") {
            return Some(Self::Sha256);
        }
        if name_lower.ends_with(".sigstore") {
            return Some(Self::Sigstore);
        }
        if name_lower.ends_with(".log") {
            return Some(Self::Log);
        }
        if name_lower.ends_with(".trace") {
            return Some(Self::Trace);
        }

        // Proibidas / restritas
        if name_lower.ends_with(".dll") {
            return Some(Self::Dll);
        }
        if name_lower.ends_with(".exe") {
            return Some(Self::Exe);
        }
        if name_lower.ends_with(".so") {
            return Some(Self::So);
        }
        if name_lower.ends_with(".dylib") {
            return Some(Self::Dylib);
        }
        if name_lower.ends_with(".pem") {
            return Some(Self::Pem);
        }
        if name_lower.ends_with(".env") {
            return Some(Self::Env);
        }
        if name_lower.ends_with(".bak") {
            return Some(Self::Bak);
        }
        if name_lower.ends_with(".tmp") {
            return Some(Self::Tmp);
        }
        if name_lower.ends_with(".dump") {
            return Some(Self::Dump);
        }

        None
    }

    /// Retorna a camada lógica associada (ou None se for extensão restrita genérica)
    pub fn layer(&self) -> Option<ArkheLayer> {
        match self {
            Self::Cid | Self::Multihash | Self::Pub | Self::Sig | Self::Cert | Self::Crl
            | Self::Keystore | Self::Seed => Some(ArkheLayer::IdentityAndContent),

            Self::Arkmanifest | Self::ArkmanifestSig | Self::Gguf | Self::GgufPart
            | Self::GgufIndex | Self::Quant | Self::Lineage | Self::Eval | Self::Dataset
            | Self::Tokenizer | Self::Vocab => Some(ArkheLayer::ManifestsAndModels),

            Self::Afir | Self::AfirJson | Self::Afirtree | Self::Wer | Self::Mmr
            | Self::Mmrproof | Self::Consistency | Self::Checkpoint | Self::Attest
            | Self::Arkbundle | Self::Wasm => Some(ArkheLayer::ReceiptsAndEvidence),

            Self::Peers | Self::Peerledger | Self::Seeds | Self::Dhtcache | Self::Announce
            | Self::Gossip | Self::Netpolicy | Self::Pcap => Some(ArkheLayer::NetworkAndDiscovery),

            Self::Policy | Self::PolicySig | Self::Request | Self::Response | Self::Session
            | Self::Router | Self::Expertmap | Self::Kv | Self::Gpu | Self::Onnx
            | Self::Safetensors => Some(ArkheLayer::InferenceAndExecution),

            Self::Fraudproof | Self::Reexec | Self::Zkproof | Self::Teequote
            | Self::Verdict | Self::Slash | Self::Golden => Some(ArkheLayer::VerificationAndFraud),

            Self::Constitution | Self::ConstitutionSig | Self::Proposal | Self::Vote
            | Self::Canon | Self::Emission | Self::Reward | Self::Stake | Self::Epoch
            | Self::Gini => Some(ArkheLayer::EconomyAndGovernance),

            Self::Anchor | Self::Ibc | Self::Ccip | Self::Layerzero | Self::Abi
            | Self::Sol | Self::Move | Self::Tx | Self::Receiptroot => Some(ArkheLayer::ChainIntegration),

            Self::Toml | Self::Rs | Self::Ts | Self::Cs | Self::Csproj | Self::Wat
            | Self::Json | Self::Lock | Self::Sbom | Self::Provenance | Self::Sha256
            | Self::Sigstore | Self::Log | Self::Trace => Some(ArkheLayer::BuildAndSupplyChain),

            Self::Dll | Self::Exe | Self::So | Self::Dylib | Self::Pem | Self::Env
            | Self::Bak | Self::Tmp | Self::Dump => None,
        }
    }

    /// Retorna o nível de obrigatoriedade normativo da extensão
    pub fn requirement(&self) -> ExtensionRequirement {
        match self {
            Self::Cid
            | Self::Pub
            | Self::Sig
            | Self::Keystore
            | Self::Arkmanifest
            | Self::ArkmanifestSig
            | Self::Gguf
            | Self::GgufPart
            | Self::Quant
            | Self::Lineage
            | Self::Eval
            | Self::Dataset
            | Self::Tokenizer
            | Self::Afir
            | Self::Afirtree
            | Self::Wer
            | Self::Mmr
            | Self::Mmrproof
            | Self::Consistency
            | Self::Checkpoint
            | Self::Attest
            | Self::Arkbundle
            | Self::Wasm
            | Self::Peers
            | Self::Peerledger
            | Self::Seeds
            | Self::Gossip
            | Self::Netpolicy
            | Self::Policy
            | Self::PolicySig
            | Self::Request
            | Self::Response
            | Self::Session
            | Self::Router
            | Self::Expertmap
            | Self::Gpu
            | Self::Fraudproof
            | Self::Reexec
            | Self::Verdict
            | Self::Slash
            | Self::Golden
            | Self::Constitution
            | Self::ConstitutionSig
            | Self::Proposal
            | Self::Vote
            | Self::Canon
            | Self::Emission
            | Self::Reward
            | Self::Stake
            | Self::Epoch
            | Self::Abi
            | Self::Receiptroot
            | Self::Toml
            | Self::Rs
            | Self::Ts
            | Self::Json
            | Self::Lock
            | Self::Sbom
            | Self::Provenance
            | Self::Sha256
            | Self::Trace => ExtensionRequirement::Mandatory,

            Self::Crl | Self::GgufIndex | Self::Gini | Self::Sigstore => {
                ExtensionRequirement::Recommended
            }

            Self::Multihash
            | Self::Cert
            | Self::Seed
            | Self::Vocab
            | Self::AfirJson
            | Self::Dhtcache
            | Self::Announce
            | Self::Pcap
            | Self::Kv
            | Self::Onnx
            | Self::Safetensors
            | Self::Zkproof
            | Self::Teequote
            | Self::Anchor
            | Self::Ibc
            | Self::Ccip
            | Self::Layerzero
            | Self::Sol
            | Self::Move
            | Self::Tx
            | Self::Cs
            | Self::Csproj
            | Self::Wat
            | Self::Log => ExtensionRequirement::Optional,

            Self::Dll | Self::Exe | Self::So | Self::Dylib | Self::Tmp => {
                ExtensionRequirement::Restricted
            }

            Self::Pem | Self::Env | Self::Bak | Self::Dump => ExtensionRequirement::Prohibited,
        }
    }

    /// Indica se o formato é content-addressed (CIDv1 / BLAKE3)
    pub fn is_content_addressed(&self) -> bool {
        matches!(
            self,
            Self::Cid
                | Self::Arkmanifest
                | Self::GgufPart
                | Self::Afir
                | Self::Afirtree
                | Self::Wer
                | Self::Checkpoint
                | Self::Arkbundle
                | Self::Constitution
                | Self::Proposal
                | Self::Golden
                | Self::Receiptroot
        )
    }

    /// Indica se o tipo de arquivo opera em modo fail-closed estrito na validação
    pub fn is_fail_closed(&self) -> bool {
        match self.requirement() {
            ExtensionRequirement::Mandatory
            | ExtensionRequirement::Restricted
            | ExtensionRequirement::Prohibited => true,
            ExtensionRequirement::Recommended | ExtensionRequirement::Optional => false,
        }
    }
}

impl fmt::Display for ArkheExtension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Erros decorrentes de validação de arquivos e integridade de extensões
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FileValidationError {
    #[error("Extensão proibida no protocolo Arkhe: '{filename}' ({extension})")]
    ProhibitedExtension {
        filename: String,
        extension: ArkheExtension,
    },
    #[error("Extensão não reconhecida e não permitida no ambiente seguro: '{filename}'")]
    UnrecognizedExtension { filename: String },
    #[error("Arquivo binário restrito não assinado ou sem proveniência atestada: '{filename}'")]
    UnattestedRestrictedBinary { filename: String },
    #[error("Vazamento de privacidade detectado em arquivo de recibo/evidência ({filename}): payload de inferência em texto claro")]
    PrivacyLeakPlainTextPayload { filename: String },
}

/// Validador normativo de arquivos fail-closed
pub struct ArkheFileValidator;

impl ArkheFileValidator {
    /// Valida se o arquivo possui extensão permitida e atende aos critérios da especificação
    pub fn validate_filename(filename: &str) -> Result<ArkheExtension, FileValidationError> {
        let ext = ArkheExtension::from_filename(filename).ok_or_else(|| {
            FileValidationError::UnrecognizedExtension {
                filename: filename.to_string(),
            }
        })?;

        match ext.requirement() {
            ExtensionRequirement::Prohibited => Err(FileValidationError::ProhibitedExtension {
                filename: filename.to_string(),
                extension: ext,
            }),
            ExtensionRequirement::Restricted => {
                if matches!(ext, ArkheExtension::Tmp) {
                    Ok(ext)
                } else {
                    // Binários nativos restritos (.dll, .exe, .so, .dylib) não fazem parte do protocolo normativo
                    Err(FileValidationError::UnattestedRestrictedBinary {
                        filename: filename.to_string(),
                    })
                }
            }
            ExtensionRequirement::Mandatory
            | ExtensionRequirement::Recommended
            | ExtensionRequirement::Optional => Ok(ext),
        }
    }

    /// Inspeciona o conteúdo de evidência/recibo para garantir que nunca contenha prompt ou output em claro
    pub fn check_evidence_privacy(filename: &str, content: &[u8]) -> Result<(), FileValidationError> {
        if let Some(ext) = ArkheExtension::from_filename(filename) {
            if matches!(
                ext,
                ArkheExtension::Afir
                    | ArkheExtension::Wer
                    | ArkheExtension::Mmr
                    | ArkheExtension::Checkpoint
                    | ArkheExtension::Arkbundle
            ) {
                // Heurística de inspeção fail-closed: verifica marcadores de prompt/raw output em texto claro
                let raw_str = String::from_utf8_lossy(content);
                if raw_str.contains("\"prompt\":") || raw_str.contains("\"raw_output\":") || raw_str.contains("\"completion\":") {
                    return Err(FileValidationError::PrivacyLeakPlainTextPayload {
                        filename: filename.to_string(),
                    });
                }
            }
        }
        Ok(())
    }
}

#[path = "extensions_tests.rs"]
#[cfg(test)]
mod tests;
