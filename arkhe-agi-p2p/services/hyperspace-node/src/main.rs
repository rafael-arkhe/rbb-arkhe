mod consensus;

#[tokio::main]
async fn main() {
    let _consensus = consensus::NodeConsensusEngine::new();
    println!("Hyperspace DAG consensus node running");
}
