use arkhe_afir::AfirReceipt;
use arkhe_crypto::verify_hybrid_envelope;
use arkhe_mobile_mmr::{LocalMmr, MobileCheckpoint};
use arkhe_mobile_wer::{MobileWER, TitanM2Binding};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BundleVerificationError {
    #[error("Binding Titan M2 inválido")]
    AttestationBindingMismatch,
    #[error("Inclusão no MMR inválida")]
    MmrInclusionFailed,
    #[error("Reescrita histórica detectada")]
    HistoricalRewriteDetected,
    #[error("Assinatura do checkpoint inválida")]
    CheckpointSignatureInvalid,
    #[error("Recibo AFiR mismatch")]
    AfirMismatch,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OfflineVerificationBundle {
    pub wers: Vec<MobileWER>,
    pub receipts: Vec<AfirReceipt>,
    pub binding: TitanM2Binding,
    pub checkpoint: MobileCheckpoint,
    pub ed25519_pk: [u8; 32],
    pub mldsa65_pk: Vec<u8>,
}

pub struct OfflineVerifier;

impl OfflineVerifier {
    pub fn verify_bundle(bundle: &OfflineVerificationBundle) -> Result<(), BundleVerificationError> {
        let computed_device_hash = bundle.binding.compute_device_attestation_hash();
        for wer in &bundle.wers {
            if wer.device_attestation_hash != computed_device_hash {
                return Err(BundleVerificationError::AttestationBindingMismatch);
            }
        }

        let mut mmr = LocalMmr::new();
        for wer in &bundle.wers {
            mmr.append(wer.entry_digest());
        }

        if mmr.bagged_root() != bundle.checkpoint.mmr_bagged_root
            || (bundle.wers.len() as u64) != bundle.checkpoint.leaf_count
        {
            return Err(BundleVerificationError::MmrInclusionFailed);
        }

        let checkpoint_digest = bundle.checkpoint.digest();
        verify_hybrid_envelope(
            &bundle.ed25519_pk,
            &bundle.mldsa65_pk,
            &checkpoint_digest,
            &bundle.checkpoint.signature_envelope,
            Some(b"STRONGBOX_CHECKPOINT_V1"),
        )
        .map_err(|_| BundleVerificationError::CheckpointSignatureInvalid)?;

        for (i, wer) in bundle.wers.iter().enumerate() {
            if let Some(receipt) = bundle.receipts.get(i) {
                if receipt.input_digest != wer.input_digest || receipt.output_digest != wer.output_digest {
                    return Err(BundleVerificationError::AfirMismatch);
                }
            }
        }
        Ok(())
    }

    pub fn detect_history_rewrite(
        bundle: &OfflineVerificationBundle,
        adulterated_index: usize,
        adulterated_wer: MobileWER,
    ) -> bool {
        let mut tampered_wers = bundle.wers.clone();
        if adulterated_index < tampered_wers.len() {
            tampered_wers[adulterated_index] = adulterated_wer;
        }

        let mut tampered_mmr = LocalMmr::new();
        for wer in &tampered_wers {
            tampered_mmr.append(wer.entry_digest());
        }

        tampered_mmr.bagged_root() != bundle.checkpoint.mmr_bagged_root
    }
}
