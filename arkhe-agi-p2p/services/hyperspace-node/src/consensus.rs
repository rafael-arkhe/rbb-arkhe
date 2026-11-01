use arkhe_hyperspace_sdk::MysticetiDagState;

pub struct NodeConsensusEngine {
    pub dag_state: MysticetiDagState,
}

impl NodeConsensusEngine {
    pub fn new() -> Self {
        Self {
            dag_state: MysticetiDagState::new(),
        }
    }
}
