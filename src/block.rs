use crate::hash::Hash256;

pub struct Block {
    timestamp: u64,
    prev_block_hash: Hash256,
    hash: Hash256,
    data: String,
}

impl Block {
    pub fn new(data: String, prev_block_hash: Hash256) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut block = Block {
            timestamp,
            prev_block_hash,
            hash: [0; 32],
            data,
        };
        
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> Hash256 {
        // We'll implement this in the hash module
        crate::hash::calculate_hash(&self.timestamp.to_string(), &self.prev_block_hash, &self.data)
    }

    pub fn genesis() -> Self {
        Block::new("Genesis Block".to_string(), [0; 32])
    }

    pub fn get_hash(&self) -> Hash256 {
        self.hash
    }

    pub fn get_prev_hash(&self) -> Hash256 {
        self.prev_block_hash
    }
}