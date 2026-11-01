use crate::cid::CidV1;
use arkhe_core::Digest32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelChunk {
    pub layer_index: usize,
    pub total_layers: usize,
    pub cid: CidV1,
    pub data: Vec<u8>,
    pub size_bytes: usize,
}

pub struct TransformerChunker {
    pub chunk_size: usize,
}

impl TransformerChunker {
    pub fn new(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    pub fn chunk_model(&self, raw_weights: &[u8]) -> Vec<ModelChunk> {
        let mut chunks = Vec::new();
        let total_size = raw_weights.len();
        let total_layers = (total_size + self.chunk_size - 1) / self.chunk_size;

        for (idx, slice) in raw_weights.chunks(self.chunk_size).enumerate() {
            let digest = Digest32::blake3(slice);
            let cid = CidV1::from_digest(digest);
            chunks.push(ModelChunk {
                layer_index: idx,
                total_layers,
                cid,
                data: slice.to_vec(),
                size_bytes: slice.len(),
            });
        }
        chunks
    }
}
