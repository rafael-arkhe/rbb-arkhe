use arkhe_core::Digest32;
use fips204::ml_dsa_65;
use fips204::traits::{KeyGen, SerDes, Signer as _, Verifier as _};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const MLDSA_65_PK_LEN: usize = 1952;
pub const MLDSA_65_SIG_LEN: usize = 3309;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Falha na assinatura Ed25519")]
    Ed25519SignError,
    #[error("Assinatura Ed25519 inválida")]
    Ed25519VerificationFailed,
    #[error("Falha na assinatura ML-DSA-65")]
    MlDsaSignError,
    #[error("Assinatura ML-DSA-65 inválida")]
    MlDsaVerificationFailed,
    #[error("Tamanho inválido de chave ou assinatura")]
    InvalidKeyLength,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HybridPublicKey {
    pub ed25519_pk: [u8; 32],
    pub mldsa65_pk: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HybridSignatureEnvelope {
    pub ed25519_sig: Vec<u8>,
    pub mldsa65_sig: Vec<u8>,
}

pub struct Ed25519Keypair {
    signing_key: ed25519_dalek::SigningKey,
}

impl Ed25519Keypair {
    pub fn generate<R: rand_core::CryptoRng + rand_core::RngCore>(rng: &mut R) -> Self {
        let signing_key = ed25519_dalek::SigningKey::generate(rng);
        Self { signing_key }
    }

    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }

    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        use ed25519_dalek::Signer;
        self.signing_key.sign(message).to_bytes()
    }

    pub fn verify(pk_bytes: &[u8; 32], message: &[u8], sig_bytes: &[u8]) -> Result<(), CryptoError> {
        let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(pk_bytes)
            .map_err(|_| CryptoError::InvalidKeyLength)?;
        let sig_arr: [u8; 64] = sig_bytes.try_into()
            .map_err(|_| CryptoError::InvalidKeyLength)?;
        let signature = ed25519_dalek::Signature::from_bytes(&sig_arr);
        verifying_key.verify_strict(message, &signature)
            .map_err(|_| CryptoError::Ed25519VerificationFailed)
    }
}

pub struct MlDsa65Keypair {
    pk: ml_dsa_65::PublicKey,
    sk: ml_dsa_65::PrivateKey,
}

impl MlDsa65Keypair {
    pub fn generate_from_seed(seed: &[u8; 32]) -> Self {
        let (pk, sk) = ml_dsa_65::KG::keygen_from_seed(seed);
        Self { pk, sk }
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.pk.clone().into_bytes().to_vec()
    }

    pub fn sign(&self, message: &[u8], ctx: Option<&[u8]>) -> Result<Vec<u8>, CryptoError> {
        let sig = self.sk.try_sign_with_seed(&[42u8; 32], message, ctx.unwrap_or(b""))
            .map_err(|_| CryptoError::MlDsaSignError)?;
        Ok(sig.to_vec())
    }

    pub fn verify(pk_bytes: &[u8], message: &[u8], sig_bytes: &[u8], ctx: Option<&[u8]>) -> Result<(), CryptoError> {
        if pk_bytes.len() != MLDSA_65_PK_LEN || sig_bytes.len() != MLDSA_65_SIG_LEN {
            return Err(CryptoError::InvalidKeyLength);
        }
        let pk_arr: [u8; MLDSA_65_PK_LEN] = pk_bytes.try_into()
            .map_err(|_| CryptoError::InvalidKeyLength)?;
        let sig_arr: [u8; MLDSA_65_SIG_LEN] = sig_bytes.try_into()
            .map_err(|_| CryptoError::InvalidKeyLength)?;
        let pk = ml_dsa_65::PublicKey::try_from_bytes(pk_arr)
            .map_err(|_| CryptoError::MlDsaVerificationFailed)?;
        if pk.verify(message, &sig_arr, ctx.unwrap_or(b"")) {
            Ok(())
        } else {
            Err(CryptoError::MlDsaVerificationFailed)
        }
    }
}

pub fn verify_hybrid_envelope(
    ed25519_pk: &[u8; 32],
    mldsa65_pk: &[u8],
    digest: &Digest32,
    envelope: &HybridSignatureEnvelope,
    ctx: Option<&[u8]>,
) -> Result<(), CryptoError> {
    Ed25519Keypair::verify(ed25519_pk, digest.as_bytes(), &envelope.ed25519_sig)?;
    MlDsa65Keypair::verify(mldsa65_pk, digest.as_bytes(), &envelope.mldsa65_sig, ctx)?;
    Ok(())
}
