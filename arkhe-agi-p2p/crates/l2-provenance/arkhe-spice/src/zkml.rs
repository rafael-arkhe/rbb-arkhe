use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZkmlProof {
    pub proof_system: String,
    pub public_inputs: Vec<Digest32>,
    pub proof_bytes: Vec<u8>,
}

impl ZkmlProof {
    pub fn verify(&self, expected_output: &Digest32) -> bool {
        !self.proof_bytes.is_empty() && self.public_inputs.contains(expected_output)
    }
}
