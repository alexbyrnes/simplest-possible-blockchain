use crate::mining::{do_work, puzzle};
use crate::model::Block;

pub fn validate(block: &Block, prevhash: u64) -> bool {
    block.data.prevhash == prevhash && puzzle(do_work(prevhash + block.data.nonce))
}
