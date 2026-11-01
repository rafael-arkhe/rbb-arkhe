use crate::action::MobileAction;
use arkhe_core::{Digest32, DOMAIN_MOBILE_ACTION};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobileWER {
    pub version: u8,
    pub device_attestation_hash: Digest32,
    pub app_id: Digest32,
    pub action: MobileAction,
    pub input_digest: Digest32,
    pub output_digest: Digest32,
    pub policy_digest: Digest32,
    pub performed_at: u64,
    pub evidence_refs: Vec<Digest32>,
}

impl MobileWER {
    pub fn entry_digest(&self) -> Digest32 {
        let serialized = serde_json::to_vec(self).unwrap_or_default();
        Digest32::blake3_domain(DOMAIN_MOBILE_ACTION, &serialized)
    }
}
