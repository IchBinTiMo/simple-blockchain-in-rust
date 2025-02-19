use sha2::{Sha256, Digest};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
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

    pub fn calculate_hash(&self) -> String {
        let record: String = format!("{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash);

        let mut h: Sha256 = Sha256::new();

        h.update(record.as_bytes());

        format!("{:x}", h.finalize())
    }

    pub fn new(data: i32, prev: &Block) -> Block {
        let index: u32 = prev.index + 1;
        let previous_hash: String = prev.hash.clone();
        let timestamp: String = Utc::now().timestamp().to_string();

        let record: String = format!("{}{}{}{}", index, timestamp, data, previous_hash);

        let mut h: Sha256 = Sha256::new();

        h.update(record.as_bytes());

        let hash: String = format!("{:x}", h.finalize());

        Block::new_block(index, hash, previous_hash, data, timestamp)
    }

    pub fn default() -> Block {
        let now: i64 = Utc::now().timestamp();

        let timestamp: String = now.to_string();

        Block::new_block(0, "".to_string(), "".to_string(), 0, timestamp)
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn get_previous_hash(&self) -> String {
        self.previous_hash.clone()
    }
}