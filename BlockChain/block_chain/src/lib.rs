//! Block submodule
//!
//! # author: Mindaugas Sharskus
//! # date: 15-02-2019
//!
//! TODO (improvements):
//! - ?? Implement merkle tree functionality fo transaction confirmation.
//! - ?? Create `BlockChainError` to handle `BlockChain` errors.
//! - hide public fields


#![allow(unused_imports)]

mod block;
pub mod block_header;
pub mod hashable;
pub mod transaction;

use serde::{Deserialize, Serialize, Serializer};
use sha2::{Digest, Sha256, Sha512};
use std::convert::AsMut;
use std::{mem, error};
use uuid::Uuid;

pub use crate::block::Block;
use crate::hashable::clone_into_array;
use crate::hashable::convert_u64_to_u8_array;
use crate::hashable::HashSha256;
use crate::hashable::Hashable;
use crate::transaction::Transaction;
use core::borrow::BorrowMut;
use std::fmt::Debug;

//////////////////////////////// Block Chain ////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockChain<T> {
    uuid: Uuid,
    pub chain: Vec<Block<T>>,
    pub pending_transactions: Vec<T>, // pending transactions
}

#[allow(dead_code)]
impl<T> BlockChain<T>
where
    T: Hashable + Debug, // Transaction<String>
{
    ///
    /// Creates new `BlockChain` with genesis block in it
    ///
    pub fn new() -> Self {
        let mut chain = Vec::new();     // create chain
        chain.push(Block::genesis());        // add genesis block to chain

        Self {
            uuid: Uuid::new_v4(),
            chain,
            pending_transactions: Vec::new(),
        }
    }

    ///
    /// Creates new "next" block.
    /// New block will have all pending transactions.
    ///
    pub fn generate_next_block(&mut self) -> Block<T> {
        // create temp next block using the last block.
        let mut new_block = self.chain
            .last()
            .unwrap_or_else(|| {
                panic!("Here is no blocks in blockchain");
            })
            .next(); // create new "next" block

        // https://doc.rust-lang.org/error-index.html#E0507
        // second part(close to end)
        // Move all pending transactions to newly created block
        mem::replace(&mut self.pending_transactions, Vec::new())
            .into_iter()
            .for_each(|tr| {
                new_block.add_record(tr);
            });

        // New next block holding all pending transactions
        new_block
    }

    ///
    /// Verify and add given `Block` to the `BlockChain`.
    /// ToDo:
    /// - check if block belongs to our `BlockChain`
    /// - check timestamp continuity
    ///
    pub fn add_block(&mut self, block: Block<T>) -> Result<&mut Self, Box<dyn error::Error>> {
        // Borrow last `Block` from `BlockChain`.
        let last_block = self.chain.last().unwrap_or_else(||{
            panic!("BlockChain fatal error! No blocks found.")
        });

        // Check for hash continuity
        if block.get_prev_hash() != &last_block.hash() {
            // If we here it's mean we are out of sync
            // or someone trying to mess with blockchain.
            panic!("Given block can't be added to current blockchain. Hash mismatch.");
        }

        // Check for index continuity
        if block.get_index() -1 != last_block.get_index() {
            panic!("Given block can't be added to current blockchain. Index mismatch.");
        }

        // If all verification passes without errors: add the block.
        self.chain.push(block);

        Ok(self)
    }

    /// Get `BlockChain` genesis `Block`
    pub fn get_block_genesis(&self) -> &Block<T> {
        &self.chain[0]
    }

    /// Get `BlockChain` last `Block`
    pub fn get_block_last(&self) -> &Block<T> {
        &self.chain.last().unwrap_or_else(||{
            panic!("BlockChain fatal error! No blocks found.")
        })
    }

    /// Get all blocks as slice starting from a given block.
    pub fn get_blocks_starting_at(&self, index: usize) -> Result<&[Block<T>], Box<dyn error::Error>> {
        Ok(&self.chain[index..])
    }

    /// Add transaction to pending transactions
    pub fn add_transaction(&mut self, transaction: T) -> &mut Self {
        self.pending_transactions.push(transaction);

        self
    }

    /// Get pending transactions
    pub fn get_pending_transactions(&self) -> &[T] {
        &self.pending_transactions.as_slice()
    }

    /// Get `BlockChain`s uuid
    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }
}

impl Hashable for String {
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(self.as_bytes());

        clone_into_array(hasher.result().as_slice())
    }
}

impl Hashable for usize {
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(convert_u64_to_u8_array(*self as u64));

        clone_into_array(hasher.result().as_slice())
    }
}

//////////////////////////////// Tests /////////////////////////////////////////////////

#[test]
fn test_blockchain_serde()  -> Result<(), Box<dyn error::Error>> {
    let mut blockchain = BlockChain::new(); // block #0 (genesis)

    println!("{:?}", blockchain);
    println!("{:?}", blockchain.get_block_last().hash());

    // crating block #1
    Transaction::new("s-1", "r-1", "message 1-1".to_string(), blockchain.borrow_mut());
    Transaction::new("s-2", "r-2", "message 2-2".to_string(), blockchain.borrow_mut());
    let block = blockchain.generate_next_block();
    blockchain.add_block(block)?;

    println!("{:?}", blockchain);

    // creating block #2
    Transaction::new("s-1", "r-2", "message 1-2".to_string(), blockchain.borrow_mut());
    Transaction::new("s-2", "r-1", "message 2-1".to_string(), blockchain.borrow_mut());
    let block = blockchain.generate_next_block();
    blockchain.add_block(block)?;

    println!("{:?}", blockchain);

    // Convert the Block to a JSON string.
    let serialized = serde_json::to_string(&blockchain).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Block.
    let deserialized: BlockChain<Transaction<String>> =
        serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

//    assert_eq!(deserialized.chain.size(), blockchain.chain.size());
        assert_eq!(deserialized.chain[1], blockchain.chain[1]);
        assert_ne!(deserialized.chain[0], blockchain.chain[1]);

    println!("{:?}", blockchain);

//    assert!(false);
    Ok(())
}

//////////////////////////////// Tests ////////////////////////////

// about mining: https://bitcoin.stackexchange.com/a/7332

// RON + SERDE : https://pastebin.com/JKPBHbNK
