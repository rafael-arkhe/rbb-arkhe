use arkhe_afir::AfirSigner;
use arkhe_crypto::{Ed25519Keypair, HybridSignatureEnvelope, MlDsa65Keypair};
use arkhe_mobile_mmr::{LocalMmr, MobileCheckpoint, StrongBoxKeyStore};
use arkhe_mobile_verify::{OfflineVerificationBundle, OfflineVerifier};
use arkhe_mobile_wer::{MobileAction, MobileWER, TitanM2Binding};
use rand::rngs::OsRng;

#[test]
fn test_offline_historical_rewrite_detection() {
    let binding = TitanM2Binding::new(
        b"TITAN_M2_TEST_QUOTE".to_vec(),
        [0x77; 32],
        arkhe_core::Digest32::blake3(b"app-measurement"),
    );
    let dev_hash = binding.compute_device_attestation_hash();

    let mut wers = Vec::new();
    let mut receipts = Vec::new();
    let mut mmr = LocalMmr::new();

    let afir_kp = MlDsa65Keypair::generate_from_seed(&[88u8; 32]);
    let afir_signer = AfirSigner::new(afir_kp);

    for i in 0..10 {
        let input_digest = arkhe_core::Digest32::blake3(format!("prompt-{}", i).as_bytes());
        let output_digest = arkhe_core::Digest32::blake3(format!("response-{}", i).as_bytes());
        let wer = MobileWER {
            version: 1,
            device_attestation_hash: dev_hash,
            app_id: arkhe_core::Digest32::blake3(b"space.arkhe.mobile"),
            action: MobileAction::InferenceExecution {
                model_cid: "bafkreib".to_string(),
            },
            input_digest,
            output_digest,
            policy_digest: arkhe_core::Digest32::ZERO,
            performed_at: 1730000000 + i as u64,
            evidence_refs: vec![],
        };
        mmr.append(wer.entry_digest());

        let receipt = afir_signer
            .sign_inference(
                arkhe_core::Digest32::ZERO,
                input_digest,
                output_digest,
                mmr.bagged_root(),
                1730000000 + i as u64,
                785_000,
            )
            .unwrap();

        wers.push(wer);
        receipts.push(receipt);
    }

    let bagged_root = mmr.bagged_root();
    let strongbox = StrongBoxKeyStore::new(&[33u8; 32]);
    let ed25519_kp = Ed25519Keypair::generate(&mut OsRng);

    let checkpoint_digest = MobileCheckpoint::compute_checkpoint_digest(
        "pixel-titan-m2",
        1,
        wers.len() as u64,
        &bagged_root,
        1730000100,
    );

    let mldsa_sig = strongbox
        .sign_in_secure_element(&checkpoint_digest)
        .unwrap();
    let ed_sig = ed25519_kp.sign(checkpoint_digest.as_bytes()).to_vec();

    let checkpoint = MobileCheckpoint {
        device_id: "pixel-titan-m2".to_string(),
        session_id: 1,
        leaf_count: wers.len() as u64,
        mmr_bagged_root: bagged_root,
        timestamp: 1730000100,
        signature_envelope: HybridSignatureEnvelope {
            ed25519_sig: ed_sig,
            mldsa65_sig: mldsa_sig,
        },
    };

    let bundle = OfflineVerificationBundle {
        wers: wers.clone(),
        receipts,
        binding,
        checkpoint,
        ed25519_pk: ed25519_kp.public_key_bytes(),
        mldsa65_pk: strongbox.public_key_bytes(),
    };

    // 1. Verificação offline sem rede é válida
    assert!(OfflineVerifier::verify_bundle(&bundle).is_ok());

    // 2. Prova de detecção de reescrita histórica offline:
    // Se um nó malicioso adulterar um MobileWER anterior (ex: no índice 3), o MMR detecta infalivelmente
    let mut tampered_wer = wers[3].clone();
    tampered_wer.output_digest = arkhe_core::Digest32::blake3(b"tampered-response");
    let rewrite_detected = OfflineVerifier::detect_history_rewrite(&bundle, 3, tampered_wer);
    assert!(
        rewrite_detected,
        "O verificador offline deve detectar reescrita do histórico que Merkle-proof pontual não detectaria"
    );
}
