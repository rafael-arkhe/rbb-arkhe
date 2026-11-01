use crate::receipt::AfirReceipt;
use arkhe_core::Digest32;
use arkhe_crypto::{CryptoError, MlDsa65Keypair};

pub struct AfirSigner {
    keypair: MlDsa65Keypair,
}

impl AfirSigner {
    pub fn new(keypair: MlDsa65Keypair) -> Self {
        Self { keypair }
    }

    pub fn sign_inference(
        &self,
        model_cid_digest: Digest32,
        input_digest: Digest32,
        output_digest: Digest32,
        inference_root: Digest32,
        executed_at: u64,
        duration_ns: u64,
    ) -> Result<AfirReceipt, CryptoError> {
        let mut receipt = AfirReceipt {
            version: 1,
            model_cid_digest,
            input_digest,
            output_digest,
            inference_root,
            executed_at,
            duration_ns,
            signer_mldsa65_pk: self.keypair.public_key_bytes(),
            mldsa65_signature: Vec::new(),
        };

        let digest = receipt.digest();
        let sig = self.keypair.sign(digest.as_bytes(), Some(b"AFIR_RECEIPT_V1"))?;
        receipt.mldsa65_signature = sig;
        Ok(receipt)
    }
}
