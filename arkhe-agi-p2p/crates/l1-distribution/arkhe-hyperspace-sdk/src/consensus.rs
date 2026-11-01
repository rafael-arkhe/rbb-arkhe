use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DagVertex {
    pub round: u64,
    pub author: String,
    pub parents: Vec<Digest32>,
    pub payload_hash: Digest32,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MysticetiDagState {
    pub current_round: u64,
    pub finalized_vertices: Vec<Digest32>,
}

impl MysticetiDagState {
    pub fn new() -> Self {
        Self { current_round: 0, finalized_vertices: Vec::new() }
    }

    pub fn commit_round(&mut self, round: u64, vertex_hash: Digest32) {
        if round >= self.current_round {
            self.current_round = round;
            self.finalized_vertices.push(vertex_hash);
        }
    }
}
