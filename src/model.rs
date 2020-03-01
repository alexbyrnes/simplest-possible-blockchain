extern crate chrono;
use chrono::{DateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Block {
    pub hash: u64,
    pub data: BlockData,
}

impl Block {
    pub fn new(data: BlockData) -> Self {
        Block {
            hash: Self::hash(&data),
            data,
        }
    }

    fn hash(data: &BlockData) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Hash)]
pub struct BlockData {
    pub index: u64,
    pub text: String,
    pub datetime: DateTime<Utc>,
    pub prevhash: u64,
    pub nonce: u64,
}

impl BlockData {
    pub fn new(index: u64, text: String, prevhash: u64, nonce: u64) -> Self {
        BlockData {
            index,
            text,
            datetime: Utc::now(),
            prevhash,
            nonce,
        }
    }
}
