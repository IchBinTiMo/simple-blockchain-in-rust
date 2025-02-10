use sha2::{Sha256, Digest};
use chrono::prelude::*;


#[derive(Debug)]
struct Block {
    index: u32,
    hash: String,
    previous_hash: String,
    data: i32,
    timestamp: String
}

impl Block {
    fn new_block(index: u32, hash: String, previous_hash: String, data: i32, timestamp: String) -> Block {
        Block {
            index,
            hash,
            previous_hash,
            data,
            timestamp
        }
    }

    fn calculate_hash(&self) -> String {
        let record: String = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash);

        let mut h: Sha256 = Sha256::new();

        h.update(record.as_bytes());

        format!("{:x}", h.finalize())
    }

    fn new(data: i32, prev: &Block) -> Block {
        let index: u32 = prev.index + 1;
        let previous_hash: String = prev.hash.clone();
        let timestamp: String = Utc::now().timestamp().to_string();

        let record: String = format!("{}{}{}{}", index, timestamp, data, previous_hash);

        let mut h: Sha256 = Sha256::new();

        h.update(record.as_bytes());

        let hash: String = format!("{:x}", h.finalize());

        Block::new_block(index, hash, previous_hash, data, timestamp)
    }
}

fn is_block_valid(current: &Block, prev: &Block) -> bool {
    current.index == prev.index + 1 && 
    current.previous_hash == prev.hash && 
    current.hash == current.calculate_hash()
}

fn main() {
    let mut blockchain: Vec<Block> = Vec::new();

    let now = Utc::now().timestamp();
    
    let genesis = Block::new_block(0, "".to_string(), "".to_string(), 0, now.to_string());
    
    blockchain.push(genesis);

    println!("{:?}", blockchain);
}
