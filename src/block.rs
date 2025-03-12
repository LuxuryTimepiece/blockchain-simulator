use sha2::{Digest, Sha256};
use chrono::Utc;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub mining_time_ms: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        Block {
            index,
            timestamp: Utc::now().timestamp(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            mining_time_ms: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.data,
            self.previous_hash,
            self.nonce,
            self.mining_time_ms
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn genesis() -> Self {
        let mut block = Block {
            index: 0,
            timestamp: 0,
            data: "Genesis Block".to_string(),
            previous_hash: "0".to_string(),
            hash: String::new(),
            nonce: 0,
            mining_time_ms: 0,
        };
        block.hash = block.calculate_hash();
        block
    }
}