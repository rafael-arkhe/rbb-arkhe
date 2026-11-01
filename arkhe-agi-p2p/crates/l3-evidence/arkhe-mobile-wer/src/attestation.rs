use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TitanM2Binding {
    pub attestation_blob: Vec<u8>,
    pub first_wer_nonce: [u8; 32],
    pub app_measurement: Digest32,
}

impl TitanM2Binding {
    pub fn new(attestation_blob: Vec<u8>, first_wer_nonce: [u8; 32], app_measurement: Digest32) -> Self {
        Self { attestation_blob, first_wer_nonce, app_measurement }
    }

    pub fn compute_device_attestation_hash(&self) -> Digest32 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"TITAN_M2_BINDING_V1");
        hasher.update(&self.attestation_blob);
        hasher.update(&self.first_wer_nonce);
        hasher.update(self.app_measurement.as_bytes());
        Digest32(*hasher.finalize().as_bytes())
    }
}
