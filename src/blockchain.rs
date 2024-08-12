use crate::block::Block;
use crate::hash::Hash256;

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
}
