use crate::cid::CidV1;
use crate::chunker::ModelChunk;
use arkhe_core::Digest32;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Chunk com CID {0} não encontrado")]
    NotFound(String),
    #[error("Corrupção detectada")]
    IntegrityMismatch,
}

pub struct MemoryChunkStore {
    chunks: HashMap<Digest32, ModelChunk>,
}

impl MemoryChunkStore {
    pub fn new() -> Self {
        Self { chunks: HashMap::new() }
    }

    pub fn insert(&mut self, chunk: ModelChunk) {
        self.chunks.insert(chunk.cid.digest, chunk);
    }

    pub fn fetch_by_cid(&self, cid: &CidV1) -> Result<ModelChunk, FetchError> {
        let chunk = self.chunks.get(&cid.digest).cloned().ok_or_else(|| FetchError::NotFound(cid.to_string()))?;
        let calculated = Digest32::blake3(&chunk.data);
        if calculated != cid.digest {
            return Err(FetchError::IntegrityMismatch);
        }
        Ok(chunk)
    }
}
