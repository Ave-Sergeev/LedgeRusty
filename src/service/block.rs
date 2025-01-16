use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::blockchain::Blockchain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub proof_of_work: u64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String) -> Self {
        Block {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash,
            hash: String::default(),
        }
    }

    fn is_valid_hash(&self, blockchain: Blockchain) -> bool {
        self.hash.starts_with(&"0".repeat(blockchain.difficulty))
    }

    fn increment_proof_of_work(&mut self) {
        self.proof_of_work += 1;
    }

    fn generate_new_hash(&mut self) {
        self.hash = self.generate_block_hash();
    }

    pub fn mine(&mut self, blockchain: Blockchain) {
        while !self.is_valid_hash(blockchain.clone()) {
            self.increment_proof_of_work();
            self.generate_new_hash();
        }
    }

    pub fn generate_block_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();

        let serialized_block_data = serde_json::to_string(&block_data).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);

        let result = hasher.finalize();

        format!("{result:x}")
    }
}
