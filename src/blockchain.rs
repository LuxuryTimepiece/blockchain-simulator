use crate::block::Block;
use log::{info, error};
use std::time::Instant;

#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 4,
        };
        blockchain.chain.push(Block::genesis());
        blockchain
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn mine_block(&mut self, data: String) -> Block {
        let start = Instant::now();
        let previous_hash = self.chain.last().map(|b| b.hash.clone()).unwrap_or_default();
        let mut block = Block::new(self.chain.len() as u64, data, previous_hash);

        // First mining loop: Find a hash that meets the difficulty
        while !block.hash.starts_with(&"0".repeat(self.difficulty)) {
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }

        // Set mining time
        let duration = start.elapsed();
        block.mining_time_ms = duration.as_millis() as u64;

        // Recompute hash with mining_time_ms and ensure difficulty is still met
        block.hash = block.calculate_hash();
        while !block.hash.starts_with(&"0".repeat(self.difficulty)) {
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }

        self.chain.push(block.clone());
        block
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.hash != current.calculate_hash() {
                error!(
                    "Hash mismatch at block {}: calculated {}, stored {}",
                    i, current.calculate_hash(), current.hash
                );
                return false;
            }
            if current.previous_hash != previous.hash {
                error!(
                    "Previous hash mismatch at block {}: expected {}, got {}",
                    i, previous.hash, current.previous_hash
                );
                return false;
            }
            if !current.hash.starts_with(&"0".repeat(self.difficulty)) {
                error!(
                    "Difficulty not met at block {}: hash {}, difficulty {}",
                    i, current.hash, self.difficulty
                );
                return false;
            }
        }
        info!("Blockchain is valid");
        true
    }
}