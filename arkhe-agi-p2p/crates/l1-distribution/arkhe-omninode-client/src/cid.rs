use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CidV1 {
    pub raw: String,
    pub digest: Digest32,
}

impl CidV1 {
    pub fn from_digest(digest: Digest32) -> Self {
        let prefix = "bafkreib";
        let hex_hash = hex::encode(digest.as_bytes());
        let raw = format!("{}{}", prefix, &hex_hash[..16]);
        Self { raw, digest }
    }

    pub fn to_string(&self) -> String {
        self.raw.clone()
    }
}

impl std::fmt::Debug for CidV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CidV1({})", self.raw)
    }
}

impl std::fmt::Display for CidV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}
