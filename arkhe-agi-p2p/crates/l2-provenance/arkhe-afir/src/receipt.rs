use arkhe_core::{Digest32, DOMAIN_AFIR_RECEIPT};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AfirReceipt {
    pub version: u8,
    pub model_cid_digest: Digest32,
    pub input_digest: Digest32,
    pub output_digest: Digest32,
    pub inference_root: Digest32,
    pub executed_at: u64,
    pub duration_ns: u64,
    pub signer_mldsa65_pk: Vec<u8>,
    pub mldsa65_signature: Vec<u8>,
}

impl AfirReceipt {
    pub fn digest(&self) -> Digest32 {
        let mut data = Vec::with_capacity(128);
        data.push(self.version);
        data.extend_from_slice(self.model_cid_digest.as_bytes());
        data.extend_from_slice(self.input_digest.as_bytes());
        data.extend_from_slice(self.output_digest.as_bytes());
        data.extend_from_slice(self.inference_root.as_bytes());
        data.extend_from_slice(&self.executed_at.to_be_bytes());
        data.extend_from_slice(&self.duration_ns.to_be_bytes());
        Digest32::blake3_domain(DOMAIN_AFIR_RECEIPT, &data)
    }
}
