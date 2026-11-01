use arkhe_core::{Digest32, DOMAIN_MOBILE_CHECKPOINT};
use arkhe_crypto::HybridSignatureEnvelope;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MobileCheckpoint {
    pub device_id: String,
    pub session_id: u64,
    pub leaf_count: u64,
    pub mmr_bagged_root: Digest32,
    pub timestamp: u64,
    pub signature_envelope: HybridSignatureEnvelope,
}

impl MobileCheckpoint {
    pub fn compute_checkpoint_digest(
        device_id: &str,
        session_id: u64,
        leaf_count: u64,
        mmr_bagged_root: &Digest32,
        timestamp: u64,
    ) -> Digest32 {
        let mut data = Vec::new();
        data.extend_from_slice(device_id.as_bytes());
        data.extend_from_slice(&session_id.to_be_bytes());
        data.extend_from_slice(&leaf_count.to_be_bytes());
        data.extend_from_slice(mmr_bagged_root.as_bytes());
        data.extend_from_slice(&timestamp.to_be_bytes());
        Digest32::blake3_domain(DOMAIN_MOBILE_CHECKPOINT, &data)
    }

    pub fn digest(&self) -> Digest32 {
        Self::compute_checkpoint_digest(
            &self.device_id,
            self.session_id,
            self.leaf_count,
            &self.mmr_bagged_root,
            self.timestamp,
        )
    }
}
