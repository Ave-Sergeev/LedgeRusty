use chrono::prelude::*;

use super::block::Block;

type Blocks = Vec<Block>;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
        };

        let mut chain = vec![];
        chain.push(genesis_block.clone());

        Blockchain {
            genesis_block,
            chain,
            difficulty,
        }
    }

    pub fn add_block(&mut self) {
        let mut new_block = Block::new(
            self.chain.len() as u64,
            self.chain[&self.chain.len() - 1].hash.clone(),
        );

        new_block.mine(self.clone());
        self.chain.push(new_block.clone());
        println!("New block added to the chain -> {new_block:?}");
    }

    pub fn get_all_blocks(&mut self) {
        let blocks = self.chain.clone();
        println!("All blocks -> {blocks:?}");
    }
}
