use crate::receipt::AfirReceipt;
use arkhe_crypto::{CryptoError, MlDsa65Keypair};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AfirVerificationError {
    #[error("Chave pública não corresponde")]
    SignerMismatch,
    #[error("Assinatura inválida: {0}")]
    SignatureInvalid(#[from] CryptoError),
}

pub struct AfirVerifier;

impl AfirVerifier {
    pub fn verify_offline(receipt: &AfirReceipt, expected_signer_pk: &[u8]) -> Result<(), AfirVerificationError> {
        if receipt.signer_mldsa65_pk != expected_signer_pk {
            return Err(AfirVerificationError::SignerMismatch);
        }
        let digest = receipt.digest();
        MlDsa65Keypair::verify(
            expected_signer_pk,
            digest.as_bytes(),
            &receipt.mldsa65_signature,
            Some(b"AFIR_RECEIPT_V1"),
        )?;
        Ok(())
    }
}
