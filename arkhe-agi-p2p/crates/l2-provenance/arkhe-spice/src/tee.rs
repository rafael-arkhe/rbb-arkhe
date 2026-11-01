use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeeQuote {
    pub enclave_id: String,
    pub measurement_hash: Digest32,
    pub quote_bytes: Vec<u8>,
    pub timestamp: u64,
}

impl TeeQuote {
    pub fn is_valid(&self, expected_measurement: &Digest32) -> bool {
        self.measurement_hash == *expected_measurement && !self.quote_bytes.is_empty()
    }
}
