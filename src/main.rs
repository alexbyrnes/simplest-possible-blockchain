mod mining;
mod model;
mod validation;

use mining::mine;
use model::{Block, BlockData};
use validation::validate;

fn main() {
    // Initialize a container for the blocks.
    let mut chain: Vec<Block> = vec![];

    // Generate a trivial genesis (first) block.
    let blockdata0 = BlockData::new(0, "genesis".to_string(), 0, 0);
    let genesis = Block::new(blockdata0);

    // Structure of a Block
    //
    // struct Block {
    //    hash: u64,
    //    data: BlockData,
    // }
    //
    // struct BlockData {
    //     index: u64,
    //     text: String, // In a cryptocurrency this would be transactions, not a String.
    //     datetime: DateTime<Utc>,
    //     prevhash: u64,
    //     nonce: u64,
    // }

    // Put the genesis block in the container (Note: the structure of the chain is
    // defined by the chain of hashes and prevhashes. Usually, blocks would be written to disk.)
    chain.push(genesis);

    // Now we have an existing blockchain (of one block) and we can start mining. In a normal
    // decentralized blockchain, the existing chain would be:
    //   1) Downloaded from other nodes in the network
    //   2) Much longer.
    // And all that follows would be in a loop: mine, validate, add a block, mine again...

    // Get the block at the end of the chain.
    let last = chain.last().unwrap();

    // Use the last hash to seed the mining puzzle and save the resulting solution. It is a nonce
    // in the cryptographic sense because it is used as input to the mining algorithm to vary the
    // output. Finding a valid block is finding a nonce that, with the previous block's hash, is a
    // solution to the mining puzzle.
    let nonce = mine(last.hash);

    // We can now choose what goes in the block since we found it. In a normal blockchain
    // we would be getting transactions broadcast from other nodes to include in the block. And we would be
    // competing with other nodes to find the solution to the mining puzzle and the right to add this
    // block. This is the core process that makes blockchain decentralization possible. For this
    // example, we're skipping consensus and pretending that we've won the right to create the next
    // block every time.
    //
    // The data to be hashed for this block includes the hash of the last block. Which means you
    // can't change any block without invalidating the entire chain (the prevhashes and hashes
    // wouldn't match anymore).
    let blockdata1 = BlockData::new(
        last.data.index + 1,
        "some transactions".to_string(),
        last.hash,
        nonce,
    );

    // Finalize the block. This means taking the hash of the BlockData contained in it and storing it as
    // a separate field.
    let block1 = Block::new(blockdata1);

    // Validate the block. Is the nonce a solution to the puzzle? Is the prevhash the previous
    // block's hash? In a normal blockchain, this block would usually come from another miner.
    // We would need to know if it's valid so we can either:
    //
    // Use it, and start mining "on top" of it (using its hash as a puzzle seed).
    //
    // Or:
    //
    // Continue mining using the existing seed.
    //
    // This is the core of Proof of Work (PoW) consensus: You believe the validation you're
    // using is generally accepted as correct, and you need it to know when you've lost the
    // current race and should start the next one.
    let valid = validate(&block1, last.hash);

    // One more time to show the pattern.
    if valid {
        chain.push(block1);

        let last2 = chain.last().unwrap();
        let nonce2 = mine(last2.hash);
        let blockdata2 = BlockData::new(
            last2.data.index + 1,
            "some more transactions".to_string(),
            last2.hash,
            nonce2,
        );
        let block2 = Block::new(blockdata2);

        let valid2 = validate(&block2, last2.hash);
        if valid2 {
            chain.push(block2);
            // and on and on... We've mined two blocks. In a normal blockchain these steps would be
            // in a loop and we would mine indefinitely.
        }
    }

    // Print out the chain.
    chain.iter().for_each(|x| println!("{:#?}", x));
}
