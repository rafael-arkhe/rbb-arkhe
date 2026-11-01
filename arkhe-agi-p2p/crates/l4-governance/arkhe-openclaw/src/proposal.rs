//! Propostas de Auto-Melhoria e Governança Descentralizada de Agentes.

use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelfImprovementProposal {
    pub proposal_id: Digest32,
    pub author_agent_id: String,
    pub patch_description: String,
    pub target_layer: usize,
    pub created_at: u64,
}

impl SelfImprovementProposal {
    pub fn new(author_agent_id: &str, patch_description: &str, target_layer: usize, created_at: u64) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"OPENCLAW_PROPOSAL_V1");
        hasher.update(author_agent_id.as_bytes());
        hasher.update(patch_description.as_bytes());
        hasher.update(&target_layer.to_be_bytes());
        hasher.update(&created_at.to_be_bytes());
        let proposal_id = Digest32(*hasher.finalize().as_bytes());

        Self {
            proposal_id,
            author_agent_id: author_agent_id.to_string(),
            patch_description: patch_description.to_string(),
            target_layer,
            created_at,
        }
    }
}
