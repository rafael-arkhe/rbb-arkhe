pub struct Libp2pServer {
    pub listen_addr: String,
}

impl Libp2pServer {
    pub fn new(addr: &str) -> Self {
        Self {
            listen_addr: addr.to_string(),
        }
    }
}
