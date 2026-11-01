use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MobileAction {
    InferenceExecution { model_cid: String },
    DecisionAuthorization { policy_rule: String },
    DataStorageCommit { partition_id: u32 },
    NetworkPacketRelay { hop_count: u8 },
}
