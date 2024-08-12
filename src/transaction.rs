use crate::hash::{calculate_hash, Hash256};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: u64,
    pub signature: Option<String>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Transaction {
            sender,
            receiver,
            amount,
            timestamp,
            signature: None,
        }
    }

    pub fn calculate_hash(&self) -> Hash256 {
        let data = format!(
            "{}{}{}{}",
            self.sender, self.receiver, self.amount, self.timestamp
        );
        calculate_hash(&self.timestamp.to_string(), &[0; 32], &data)
    }

    pub fn sign(&mut self, signature: String) {
        self.signature = Some(signature);
    }

    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }

    pub fn to_string(&self) -> String {
        format!(
            "From: {} To: {} Amount: {} Time: {} Signed: {}",
            self.sender,
            self.receiver,
            self.amount,
            self.timestamp,
            self.is_signed()
        )
    }

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
