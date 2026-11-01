//! Registro de Reputação de Agentes P2P.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AgentReputationRegistry {
    pub scores: HashMap<String, f64>,
}

impl AgentReputationRegistry {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    pub fn record_action(&mut self, agent_id: &str, valid: bool) {
        let score = self.scores.entry(agent_id.to_string()).or_insert(1.0);
        if valid {
            *score = (*score * 1.05).min(10.0);
        } else {
            *score = (*score * 0.5).max(0.0);
        }
    }

    pub fn get_score(&self, agent_id: &str) -> f64 {
        self.scores.get(agent_id).copied().unwrap_or(1.0)
    }
}
