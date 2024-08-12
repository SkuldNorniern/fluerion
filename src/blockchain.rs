use crate::block::Block;
use crate::hash::{Hash256, hash_to_hex};

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data, prev_block.get_hash());
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1];

            if current_block.get_hash() != current_block.calculate_hash() {
                return false;
            }

            if current_block.get_prev_hash() != prev_block.get_hash() {
                return false;
            }
        }
        true
    }

    pub fn get_latest_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    pub fn print_chain(&self) {
        for (index, block) in self.blocks.iter().enumerate() {
            println!("Block #{}", index);
            println!("Timestamp: {}", block.get_timestamp());
            println!("Previous Hash: {}", hash_to_hex(&block.get_prev_hash()));
            println!("Hash: {}", hash_to_hex(&block.get_hash()));
            println!("Data: {}", block.get_data());
            println!("------------------------");
        }
    }
}