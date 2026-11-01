use arkhe_core::Digest32;
use arkhe_crypto::{CryptoError, MlDsa65Keypair};

pub struct StrongBoxKeyStore {
    mldsa_keypair: MlDsa65Keypair,
}

impl StrongBoxKeyStore {
    pub fn new(seed: &[u8; 32]) -> Self {
        let mldsa_keypair = MlDsa65Keypair::generate_from_seed(seed);
        Self { mldsa_keypair }
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.mldsa_keypair.public_key_bytes()
    }

    pub fn sign_in_secure_element(&self, digest: &Digest32) -> Result<Vec<u8>, CryptoError> {
        self.mldsa_keypair.sign(digest.as_bytes(), Some(b"STRONGBOX_CHECKPOINT_V1"))
    }
}
