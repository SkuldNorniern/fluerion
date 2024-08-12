use crate::block::Block;
use crate::hash::{Hash256, hash_to_hex, calculate_hash};
use crate::transaction::Transaction;

const DIFFICULTY: usize = 4; // Number of leading zeros required in the hash

pub struct Blockchain {
    blocks: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Block::genesis()],
            pending_transactions: Vec::new(),
            difficulty: DIFFICULTY,
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self) {
        let new_block = self.new_block();
        self.blocks.push(new_block);
        self.pending_transactions.clear();
    }

    pub fn new_block(&self) -> Block {
        let prev_block = self.get_latest_block();
        let mut new_block = Block::new(
            self.pending_transactions.clone(),
            prev_block.get_hash(),
        );

        let (nonce, hash) = self.proof_of_work(&new_block);
        new_block.set_nonce(nonce);
        new_block.set_hash(hash);

        new_block
    }

    pub fn proof_of_work(&self, block: &Block) -> (u64, Hash256) {
        let mut nonce = 0;
        loop {
            let hash = self.calculate_hash_with_nonce(block, nonce);
            if self.valid_proof(&hash) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }

    fn calculate_hash_with_nonce(&self, block: &Block, nonce: u64) -> Hash256 {
        let data = block.get_transactions().iter()
            .map(|tx| tx.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        calculate_hash(
            &block.get_timestamp().to_string(),
            &block.get_prev_hash(),
            &format!("{}{}", data, nonce),
        )
    }

    pub fn valid_proof(&self, hash: &Hash256) -> bool {
        let hash_str = hash_to_hex(hash);
        hash_str.starts_with(&"0".repeat(self.difficulty))
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

            if !self.valid_proof(&current_block.get_hash()) {
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
            println!("Nonce: {}", block.get_nonce());
            println!("Transactions:");
            for tx in block.get_transactions() {
                println!("  {}", tx.to_string());
            }
            println!("------------------------");
        }
    }
}