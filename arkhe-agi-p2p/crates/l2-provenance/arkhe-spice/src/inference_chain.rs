use arkhe_core::{Digest32, DOMAIN_SPICE_LEAF, DOMAIN_SPICE_NODE};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InferenceChainLeaf {
    pub step: u64,
    pub fragment_hash: Digest32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InferenceChain {
    pub leaves: Vec<InferenceChainLeaf>,
}

impl InferenceChain {
    pub fn new() -> Self {
        Self { leaves: Vec::new() }
    }

    pub fn append(&mut self, fragment_hash: Digest32) {
        let step = self.leaves.len() as u64;
        self.leaves.push(InferenceChainLeaf { step, fragment_hash });
    }

    pub fn root(&self) -> Digest32 {
        if self.leaves.is_empty() {
            return Digest32::ZERO;
        }

        let mut current_level: Vec<Digest32> = self.leaves.iter().map(|leaf| {
            let mut data = leaf.step.to_be_bytes().to_vec();
            data.extend_from_slice(leaf.fragment_hash.as_bytes());
            Digest32::blake3_domain(DOMAIN_SPICE_LEAF, &data)
        }).collect();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                if chunk.len() == 2 {
                    let mut node_data = chunk[0].as_bytes().to_vec();
                    node_data.extend_from_slice(chunk[1].as_bytes());
                    next_level.push(Digest32::blake3_domain(DOMAIN_SPICE_NODE, &node_data));
                } else {
                    next_level.push(chunk[0]);
                }
            }
            current_level = next_level;
        }

        current_level[0]
    }
}
