use arkhe_core::Digest32;
use arkhe_crypto::{Ed25519Keypair, MlDsa65Keypair};
use rand::rngs::OsRng;

pub struct TestIdentity {
    pub name: String,
    pub ed25519: Ed25519Keypair,
    pub mldsa65: MlDsa65Keypair,
}

impl TestIdentity {
    pub fn new(name: &str) -> Self {
        let ed25519 = Ed25519Keypair::generate(&mut OsRng);
        let mut seed = [0u8; 32];
        let bytes = name.as_bytes();
        let copy_len = bytes.len().min(32);
        seed[..copy_len].copy_from_slice(&bytes[..copy_len]);
        let mldsa65 = MlDsa65Keypair::generate_from_seed(&seed);
        Self {
            name: name.to_string(),
            ed25519,
            mldsa65,
        }
    }
}

pub fn mock_digest(seed: u8) -> Digest32 {
    let mut buf = [seed; 32];
    buf[0] = 0xAA;
    Digest32::blake3(&buf)
}

pub fn mock_titan_m2_attestation(nonce: &[u8]) -> Vec<u8> {
    let mut quote = b"TITAN_M2_SEALED_ATTESTATION:BOOT_STATE=LOCKED:OS_PATCH=20261101:NONCE=".to_vec();
    quote.extend_from_slice(nonce);
    quote
}
