use arkhe_mobile_wer::{MobileAction, MobileWER, TitanM2Binding};

#[test]
fn test_l2_to_l3_titan_m2_binding() {
    let binding = TitanM2Binding::new(
        b"TITAN_M2_SEALED_ATTESTATION_BLOB".to_vec(),
        [0x99; 32],
        arkhe_core::Digest32::blake3(b"app-apk-signature"),
    );
    let dev_hash = binding.compute_device_attestation_hash();

    let wer = MobileWER {
        version: 1,
        device_attestation_hash: dev_hash,
        app_id: arkhe_core::Digest32::blake3(b"space.arkhe.mobile"),
        action: MobileAction::InferenceExecution {
            model_cid: "bafkreib123".to_string(),
        },
        input_digest: arkhe_core::Digest32::blake3(b"input"),
        output_digest: arkhe_core::Digest32::blake3(b"output"),
        policy_digest: arkhe_core::Digest32::blake3(b"policy"),
        performed_at: 1730000000,
        evidence_refs: vec![],
    };

    assert_eq!(wer.device_attestation_hash, dev_hash);
}
