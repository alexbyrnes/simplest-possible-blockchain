use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn mine(hash: u64) -> u64 {
    let mut nonce = 0;
    let mut solved = false;
    let mut proposal;

    while !solved {
        proposal = do_work(hash + nonce);
        solved = puzzle(proposal);
        if !solved {
            nonce += 1;
        }
    }
    nonce
}

pub fn do_work(data: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

pub fn puzzle(proposal: u64) -> bool {
    proposal % 80 == 0
}
