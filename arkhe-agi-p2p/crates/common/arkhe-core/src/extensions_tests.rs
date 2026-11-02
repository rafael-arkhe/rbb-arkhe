#[cfg(test)]
mod tests {
    use crate::extensions::*;

    #[test]
    fn test_layer_1_identity_and_content() {
        assert_eq!(ArkheExtension::from_filename("model.cid"), Some(ArkheExtension::Cid));
        assert_eq!(ArkheExtension::from_filename("digest.multihash"), Some(ArkheExtension::Multihash));
        assert_eq!(ArkheExtension::from_filename("publisher.pub"), Some(ArkheExtension::Pub));
        assert_eq!(ArkheExtension::from_filename("manifest.sig"), Some(ArkheExtension::Sig));
        assert_eq!(ArkheExtension::from_filename("device.cert"), Some(ArkheExtension::Cert));
        assert_eq!(ArkheExtension::from_filename("revocation.crl"), Some(ArkheExtension::Crl));
        assert_eq!(ArkheExtension::from_filename("node.keystore"), Some(ArkheExtension::Keystore));
        assert_eq!(ArkheExtension::from_filename("node.seed"), Some(ArkheExtension::Seed));

        assert_eq!(ArkheExtension::Cid.layer(), Some(ArkheLayer::IdentityAndContent));
        assert_eq!(ArkheExtension::Cid.requirement(), ExtensionRequirement::Mandatory);
        assert!(ArkheExtension::Cid.is_content_addressed());
    }

    #[test]
    fn test_layer_2_manifests_and_models() {
        assert_eq!(ArkheExtension::from_filename("model.arkmanifest"), Some(ArkheExtension::Arkmanifest));
        assert_eq!(ArkheExtension::from_filename("model.arkmanifest.sig"), Some(ArkheExtension::ArkmanifestSig));
        assert_eq!(ArkheExtension::from_filename("model.gguf"), Some(ArkheExtension::Gguf));
        assert_eq!(ArkheExtension::from_filename("model.gguf.part0001"), Some(ArkheExtension::GgufPart));
        assert_eq!(ArkheExtension::from_filename("model.gguf.index"), Some(ArkheExtension::GgufIndex));
        assert_eq!(ArkheExtension::from_filename("nvfp4.quant"), Some(ArkheExtension::Quant));
        assert_eq!(ArkheExtension::from_filename("training.lineage"), Some(ArkheExtension::Lineage));
        assert_eq!(ArkheExtension::from_filename("evaluation.eval"), Some(ArkheExtension::Eval));
        assert_eq!(ArkheExtension::from_filename("cohort.dataset"), Some(ArkheExtension::Dataset));
        assert_eq!(ArkheExtension::from_filename("tokenizer.json"), Some(ArkheExtension::Tokenizer));
        assert_eq!(ArkheExtension::from_filename("vocab.txt"), Some(ArkheExtension::Vocab));

        assert_eq!(ArkheExtension::Arkmanifest.layer(), Some(ArkheLayer::ManifestsAndModels));
        assert_eq!(ArkheExtension::GgufPart.layer(), Some(ArkheLayer::ManifestsAndModels));
        assert!(ArkheExtension::Arkmanifest.is_content_addressed());
    }

    #[test]
    fn test_layer_3_receipts_and_evidence() {
        assert_eq!(ArkheExtension::from_filename("fragment.afir"), Some(ArkheExtension::Afir));
        assert_eq!(ArkheExtension::from_filename("fragment.afir.json"), Some(ArkheExtension::AfirJson));
        assert_eq!(ArkheExtension::from_filename("session.afirtree"), Some(ArkheExtension::Afirtree));
        assert_eq!(ArkheExtension::from_filename("session.wer"), Some(ArkheExtension::Wer));
        assert_eq!(ArkheExtension::from_filename("device.mmr"), Some(ArkheExtension::Mmr));
        assert_eq!(ArkheExtension::from_filename("inclusion.mmrproof"), Some(ArkheExtension::Mmrproof));
        assert_eq!(ArkheExtension::from_filename("history.consistency"), Some(ArkheExtension::Consistency));
        assert_eq!(ArkheExtension::from_filename("device.checkpoint"), Some(ArkheExtension::Checkpoint));
        assert_eq!(ArkheExtension::from_filename("boot.attest"), Some(ArkheExtension::Attest));
        assert_eq!(ArkheExtension::from_filename("audit.arkbundle"), Some(ArkheExtension::Arkbundle));
        assert_eq!(ArkheExtension::from_filename("arkverify.wasm"), Some(ArkheExtension::Wasm));

        assert_eq!(ArkheExtension::Afir.layer(), Some(ArkheLayer::ReceiptsAndEvidence));
        assert_eq!(ArkheExtension::Arkbundle.layer(), Some(ArkheLayer::ReceiptsAndEvidence));
        assert!(ArkheExtension::Afir.is_content_addressed());
        assert!(ArkheExtension::Arkbundle.is_content_addressed());
    }

    #[test]
    fn test_layer_4_network_and_discovery() {
        assert_eq!(ArkheExtension::from_filename("peers.dat"), Some(ArkheExtension::Peers));
        assert_eq!(ArkheExtension::from_filename("service.peerledger"), Some(ArkheExtension::Peerledger));
        assert_eq!(ArkheExtension::from_filename("seeds.toml"), Some(ArkheExtension::Seeds));
        assert_eq!(ArkheExtension::from_filename("dht.cache"), Some(ArkheExtension::Dhtcache));
        assert_eq!(ArkheExtension::from_filename("chunk.announce"), Some(ArkheExtension::Announce));
        assert_eq!(ArkheExtension::from_filename("topic.gossip"), Some(ArkheExtension::Gossip));
        assert_eq!(ArkheExtension::from_filename("network.netpolicy"), Some(ArkheExtension::Netpolicy));
        assert_eq!(ArkheExtension::from_filename("debug.pcap"), Some(ArkheExtension::Pcap));

        assert_eq!(ArkheExtension::Peers.layer(), Some(ArkheLayer::NetworkAndDiscovery));
        assert!(ArkheExtension::Peers.is_fail_closed());
    }

    #[test]
    fn test_layer_5_inference_and_execution() {
        assert_eq!(ArkheExtension::from_filename("inference.policy"), Some(ArkheExtension::Policy));
        assert_eq!(ArkheExtension::from_filename("inference.policy.sig"), Some(ArkheExtension::PolicySig));
        assert_eq!(ArkheExtension::from_filename("inference.request"), Some(ArkheExtension::Request));
        assert_eq!(ArkheExtension::from_filename("inference.response"), Some(ArkheExtension::Response));
        assert_eq!(ArkheExtension::from_filename("inference.session"), Some(ArkheExtension::Session));
        assert_eq!(ArkheExtension::from_filename("moe.router"), Some(ArkheExtension::Router));
        assert_eq!(ArkheExtension::from_filename("moe.expertmap"), Some(ArkheExtension::Expertmap));
        assert_eq!(ArkheExtension::from_filename("kvcache.bin"), Some(ArkheExtension::Kv));
        assert_eq!(ArkheExtension::from_filename("gpu.profile"), Some(ArkheExtension::Gpu));
        assert_eq!(ArkheExtension::from_filename("model.onnx"), Some(ArkheExtension::Onnx));
        assert_eq!(ArkheExtension::from_filename("model.safetensors"), Some(ArkheExtension::Safetensors));

        assert_eq!(ArkheExtension::Policy.layer(), Some(ArkheLayer::InferenceAndExecution));
    }

    #[test]
    fn test_layer_6_verification_and_fraud() {
        assert_eq!(ArkheExtension::from_filename("receipt.fraudproof"), Some(ArkheExtension::Fraudproof));
        assert_eq!(ArkheExtension::from_filename("reexecution.reexec"), Some(ArkheExtension::Reexec));
        assert_eq!(ArkheExtension::from_filename("fragment.zkproof"), Some(ArkheExtension::Zkproof));
        assert_eq!(ArkheExtension::from_filename("attestation.teequote"), Some(ArkheExtension::Teequote));
        assert_eq!(ArkheExtension::from_filename("adjudication.verdict"), Some(ArkheExtension::Verdict));
        assert_eq!(ArkheExtension::from_filename("slashing.slash"), Some(ArkheExtension::Slash));
        assert_eq!(ArkheExtension::from_filename("vectors.golden"), Some(ArkheExtension::Golden));

        assert_eq!(ArkheExtension::Fraudproof.layer(), Some(ArkheLayer::VerificationAndFraud));
        assert!(ArkheExtension::Golden.is_content_addressed());
    }

    #[test]
    fn test_layer_7_economy_and_governance() {
        assert_eq!(ArkheExtension::from_filename("arkhe.constitution"), Some(ArkheExtension::Constitution));
        assert_eq!(ArkheExtension::from_filename("arkhe.constitution.sig"), Some(ArkheExtension::ConstitutionSig));
        assert_eq!(ArkheExtension::from_filename("upgrade.proposal"), Some(ArkheExtension::Proposal));
        assert_eq!(ArkheExtension::from_filename("quorum.vote"), Some(ArkheExtension::Vote));
        assert_eq!(ArkheExtension::from_filename("canonification.canon"), Some(ArkheExtension::Canon));
        assert_eq!(ArkheExtension::from_filename("emission.schedule"), Some(ArkheExtension::Emission));
        assert_eq!(ArkheExtension::from_filename("poi.reward"), Some(ArkheExtension::Reward));
        assert_eq!(ArkheExtension::from_filename("validator.stake"), Some(ArkheExtension::Stake));
        assert_eq!(ArkheExtension::from_filename("roles.epoch"), Some(ArkheExtension::Epoch));
        assert_eq!(ArkheExtension::from_filename("emission.gini"), Some(ArkheExtension::Gini));

        assert_eq!(ArkheExtension::Constitution.layer(), Some(ArkheLayer::EconomyAndGovernance));
        assert!(ArkheExtension::Constitution.is_content_addressed());
    }

    #[test]
    fn test_layer_8_chain_integration() {
        assert_eq!(ArkheExtension::from_filename("checkpoint.anchor"), Some(ArkheExtension::Anchor));
        assert_eq!(ArkheExtension::from_filename("cal.ibc"), Some(ArkheExtension::Ibc));
        assert_eq!(ArkheExtension::from_filename("cal.ccip"), Some(ArkheExtension::Ccip));
        assert_eq!(ArkheExtension::from_filename("cal.layerzero"), Some(ArkheExtension::Layerzero));
        assert_eq!(ArkheExtension::from_filename("ISettlement.abi"), Some(ArkheExtension::Abi));
        assert_eq!(ArkheExtension::from_filename("ArkheSettlement.sol"), Some(ArkheExtension::Sol));
        assert_eq!(ArkheExtension::from_filename("arkhe.move"), Some(ArkheExtension::Move));
        assert_eq!(ArkheExtension::from_filename("anchor.tx"), Some(ArkheExtension::Tx));
        assert_eq!(ArkheExtension::from_filename("settlement.receiptroot"), Some(ArkheExtension::Receiptroot));

        assert_eq!(ArkheExtension::Receiptroot.layer(), Some(ArkheLayer::ChainIntegration));
        assert!(ArkheExtension::Receiptroot.is_content_addressed());
    }

    #[test]
    fn test_layer_9_build_and_supply_chain() {
        assert_eq!(ArkheExtension::from_filename("Cargo.toml"), Some(ArkheExtension::Toml));
        assert_eq!(ArkheExtension::from_filename("lib.rs"), Some(ArkheExtension::Rs));
        assert_eq!(ArkheExtension::from_filename("index.ts"), Some(ArkheExtension::Ts));
        assert_eq!(ArkheExtension::from_filename("Node.cs"), Some(ArkheExtension::Cs));
        assert_eq!(ArkheExtension::from_filename("Arkhe.csproj"), Some(ArkheExtension::Csproj));
        assert_eq!(ArkheExtension::from_filename("arkverify.wat"), Some(ArkheExtension::Wat));
        assert_eq!(ArkheExtension::from_filename("package.json"), Some(ArkheExtension::Json));
        assert_eq!(ArkheExtension::from_filename("Cargo.lock"), Some(ArkheExtension::Lock));
        assert_eq!(ArkheExtension::from_filename("release.sbom"), Some(ArkheExtension::Sbom));
        assert_eq!(ArkheExtension::from_filename("build.provenance"), Some(ArkheExtension::Provenance));
        assert_eq!(ArkheExtension::from_filename("artifact.sha256"), Some(ArkheExtension::Sha256));
        assert_eq!(ArkheExtension::from_filename("artifact.sigstore"), Some(ArkheExtension::Sigstore));
        assert_eq!(ArkheExtension::from_filename("node.log"), Some(ArkheExtension::Log));
        assert_eq!(ArkheExtension::from_filename("consensus.trace"), Some(ArkheExtension::Trace));

        assert_eq!(ArkheExtension::Sbom.layer(), Some(ArkheLayer::BuildAndSupplyChain));
    }

    #[test]
    fn test_fail_closed_validation_and_prohibited_extensions() {
        // Arquivos válidos
        assert!(ArkheFileValidator::validate_filename("model.arkmanifest").is_ok());
        assert!(ArkheFileValidator::validate_filename("fragment.afir").is_ok());
        assert!(ArkheFileValidator::validate_filename("audit.arkbundle").is_ok());
        assert!(ArkheFileValidator::validate_filename("receipt.tmp").is_ok());

        // Proibidos estritos
        assert_eq!(
            ArkheFileValidator::validate_filename("private.pem"),
            Err(FileValidationError::ProhibitedExtension {
                filename: "private.pem".into(),
                extension: ArkheExtension::Pem,
            })
        );
        assert_eq!(
            ArkheFileValidator::validate_filename(".env"),
            Err(FileValidationError::ProhibitedExtension {
                filename: ".env".into(),
                extension: ArkheExtension::Env,
            })
        );
        assert_eq!(
            ArkheFileValidator::validate_filename("state.bak"),
            Err(FileValidationError::ProhibitedExtension {
                filename: "state.bak".into(),
                extension: ArkheExtension::Bak,
            })
        );
        assert_eq!(
            ArkheFileValidator::validate_filename("payload.dump"),
            Err(FileValidationError::ProhibitedExtension {
                filename: "payload.dump".into(),
                extension: ArkheExtension::Dump,
            })
        );

        // Restritos não atestados
        assert_eq!(
            ArkheFileValidator::validate_filename("payload.dll"),
            Err(FileValidationError::UnattestedRestrictedBinary {
                filename: "payload.dll".into(),
            })
        );
        assert_eq!(
            ArkheFileValidator::validate_filename("binary.exe"),
            Err(FileValidationError::UnattestedRestrictedBinary {
                filename: "binary.exe".into(),
            })
        );

        // Desconhecidos
        assert_eq!(
            ArkheFileValidator::validate_filename("unknown.xyz"),
            Err(FileValidationError::UnrecognizedExtension {
                filename: "unknown.xyz".into(),
            })
        );
    }

    #[test]
    fn test_privacy_leak_detection() {
        let safe_payload = b"{\"hash\":\"1234567890abcdef\",\"digest\":\"bafkreib123\"}";
        assert!(ArkheFileValidator::check_evidence_privacy("test.afir", safe_payload).is_ok());

        let leaking_payload = b"{\"hash\":\"1234567890abcdef\",\"prompt\":\"Secret medical record\"}";
        assert_eq!(
            ArkheFileValidator::check_evidence_privacy("test.afir", leaking_payload),
            Err(FileValidationError::PrivacyLeakPlainTextPayload {
                filename: "test.afir".into()
            })
        );
    }
}
