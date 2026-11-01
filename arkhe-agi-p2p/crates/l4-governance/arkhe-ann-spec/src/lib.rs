//! Especificação formal A.N.N. (Autonomous Neural Network):
//! Ausência total de autoridade central de agregação de pesos;
//! Verificação contra acumulador de enrolamento quase-periódico.

use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuasiPeriodicAccumulator {
    pub epoch: u64,
    pub state_digest: Digest32,
    pub weights_root: Digest32,
}

impl QuasiPeriodicAccumulator {
    pub fn new(epoch: u64, weights_root: Digest32) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"ANN_QUASI_PERIODIC_V1");
        hasher.update(&epoch.to_be_bytes());
        hasher.update(weights_root.as_bytes());
        let state_digest = Digest32(*hasher.finalize().as_bytes());
        Self {
            epoch,
            state_digest,
            weights_root,
        }
    }

    pub fn verify_state(&self, declared_digest: &Digest32) -> bool {
        &self.state_digest == declared_digest
    }
}
