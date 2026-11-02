use serde::{Deserialize, Serialize};
use std::fmt;

pub mod extensions;
pub use extensions::{
    ArkheExtension, ArkheFileValidator, ArkheLayer, ExtensionRequirement, FileValidationError,
};

pub const DOMAIN_MOBILE_ACTION: &[u8] = b"AKG:MOBILE:ACT:v1\0";
pub const DOMAIN_MOBILE_CHECKPOINT: &[u8] = b"AKG:MOBILE:CHECKPOINT:v1\0";
pub const DOMAIN_AFIR_RECEIPT: &[u8] = b"AKG:AFIR:RECEIPT:v1\0";
pub const DOMAIN_SPICE_LEAF: &[u8] = b"AKG:SPICE:LEAF:v1\0";
pub const DOMAIN_SPICE_NODE: &[u8] = b"AKG:SPICE:NODE:v1\0";

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Digest32(pub [u8; 32]);

impl Digest32 {
    pub const ZERO: Self = Self([0u8; 32]);

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn blake3(data: &[u8]) -> Self {
        let hash = blake3::hash(data);
        Self(*hash.as_bytes())
    }

    pub fn blake3_domain(domain: &[u8], data: &[u8]) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(domain);
        hasher.update(data);
        Self(*hasher.finalize().as_bytes())
    }
}

impl fmt::Debug for Digest32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Digest32({})", hex::encode(self.0))
    }
}

impl fmt::Display for Digest32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl AsRef<[u8]> for Digest32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
