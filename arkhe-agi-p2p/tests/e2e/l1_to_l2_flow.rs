use arkhe_afir::{AfirSigner, AfirVerifier};
use arkhe_crypto::MlDsa65Keypair;
use arkhe_omninode_client::TransformerChunker;

#[test]
fn test_l1_omninode_to_l2_afir_flow() {
    let chunker = TransformerChunker::new(512);
    let fake_weights = vec![0x42; 2048];
    let chunks = chunker.chunk_model(&fake_weights);
    assert_eq!(chunks.len(), 4);

    let keypair = MlDsa65Keypair::generate_from_seed(&[55u8; 32]);
    let pk = keypair.public_key_bytes();
    let signer = AfirSigner::new(keypair);

    let receipt = signer
        .sign_inference(
            chunks[0].cid.digest,
            arkhe_core::Digest32::blake3(b"user-prompt"),
            arkhe_core::Digest32::blake3(b"output-token"),
            arkhe_core::Digest32::blake3(b"spice-root"),
            1730000000,
            785_000,
        )
        .expect("sign inference receipt");

    assert!(AfirVerifier::verify_offline(&receipt, &pk).is_ok());
}
