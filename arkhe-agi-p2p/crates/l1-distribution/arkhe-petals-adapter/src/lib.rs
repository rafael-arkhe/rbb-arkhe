use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwarmNode {
    pub peer_id: String,
    pub assigned_layers: (usize, usize),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PetalsInferencePipeline {
    pub model_id: String,
    pub swarm_nodes: Vec<SwarmNode>,
}

impl PetalsInferencePipeline {
    pub fn new(model_id: &str) -> Self {
        Self { model_id: model_id.to_string(), swarm_nodes: Vec::new() }
    }

    pub fn register_peer(&mut self, peer_id: &str, start_layer: usize, end_layer: usize) {
        self.swarm_nodes.push(SwarmNode {
            peer_id: peer_id.to_string(),
            assigned_layers: (start_layer, end_layer),
        });
    }

    pub fn execute_untrusted_step(&self, input: &[u8]) -> (Vec<u8>, Digest32) {
        let mut output = input.to_vec();
        output.extend_from_slice(b":processed_by_petals_swarm");
        let hash = Digest32::blake3(&output);
        (output, hash)
    }
}
