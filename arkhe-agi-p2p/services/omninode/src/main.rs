mod server;
mod storage;

#[tokio::main]
async fn main() {
    let _storage = storage::ChunkStorageService::new();
    let _server = server::Libp2pServer::new("/ip4/0.0.0.0/tcp/4001");
    println!("OmniNode storage and distribution daemon running on port 4001");
}
