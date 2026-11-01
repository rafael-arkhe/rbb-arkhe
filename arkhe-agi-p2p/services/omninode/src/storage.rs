use arkhe_omninode_client::MemoryChunkStore;

pub struct ChunkStorageService {
    pub store: MemoryChunkStore,
}

impl ChunkStorageService {
    pub fn new() -> Self {
        Self {
            store: MemoryChunkStore::new(),
        }
    }
}
