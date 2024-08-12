use serde::{Deserialize, Serialize};

use crate::hash::Hash256;
use crate::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    timestamp: u64,
    prev_block_hash: Hash256,
    hash: Hash256,
    transactions: Vec<Transaction>,
    nonce: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_block_hash: Hash256) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut block = Block {
            timestamp,
            prev_block_hash,
            hash: [0; 32],
            transactions,
            nonce: 0,
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> Hash256 {
        let data = self
            .transactions
            .iter()
            .map(|tx| tx.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        crate::hash::calculate_hash(&self.timestamp.to_string(), &self.prev_block_hash, &data)
    }

    pub fn genesis() -> Self {
        let genesis_tx = Transaction::new("Genesis".to_string(), "Genesis".to_string(), 0.0);
        Block::new(vec![genesis_tx], [0; 32])
    }

    pub fn get_hash(&self) -> Hash256 {
        self.hash
    }

    pub fn get_prev_hash(&self) -> Hash256 {
        self.prev_block_hash
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }

    pub fn set_hash(&mut self, hash: Hash256) {
        self.hash = hash;
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
