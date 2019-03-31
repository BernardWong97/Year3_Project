//#![allow(non_snake_case)]
#![allow(unused_imports)]


//! Block submodule
//!
//! # author: Mindaugas Sharskus
//! # date: 15-20-2019
//!

mod block;
pub mod block_header;
pub mod hashable;
pub mod transaction;

use serde::{Deserialize, Serialize, Serializer};
use sha2::{Digest, Sha256, Sha512};
use std::convert::AsMut;
use std::mem;
use uuid::Uuid;

pub use crate::block::Block;
use crate::hashable::clone_into_array;
use crate::hashable::convert_u64_to_u8_array;
use crate::hashable::HashSha256;
use crate::hashable::Hashable;
use crate::transaction::Transaction;

//////////////////////////////// Block Chain ////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockChain<T> {
    uuid: Uuid,
    chain: Vec<Block<T>>,
    transactions: Vec<T>, // pending transactions
}

#[allow(dead_code)]
impl<T> BlockChain<T>
where
    T: Hashable,
{
    // TODO: Implement merkle tree functionality fo transaction confirmation.
    ///
    /// Creates new blockchain with genesis block in it
    ///
    pub fn new() -> Self {
        let mut chain = Vec::new();     // create chain
        chain.push(Block::genesis());        // add genesis block to chain

        Self {
            uuid: Uuid::new_v4(),
            chain,
            transactions: Vec::new(),
        }
    }

    ///
    /// Creates new "next" block.
    /// New block will have all pending transactions.
    /// TODO: rewrite it
    ///
    pub fn create_next_block(&mut self) -> &Block<T> {
        let mut new_block = self
            .chain
            .last()     // TODO: check if element is removed from DB
            .unwrap_or_else(|| {
                panic!("Here is no blocks in blockchain");
            })
            .next(); // create new "next" block

        // add all pending transactions to new block //

        // https://doc.rust-lang.org/error-index.html#E0507
        // second part(close to end)
        mem::replace(&mut self.transactions, Vec::new())
            .into_iter()
            .for_each(|tr| {
                new_block.add_record(tr);
            });

        assert_eq!(self.transactions.len(), 0);

        // add new block to blockchain
        self.chain.push(new_block);

        // return blockchain last block reference
        &self
            .chain
            .last()
            .unwrap_or_else(|| panic!("Here is no blocks in blockchain"))
    }

    ///
    /// Add transaction to pending transactions
    ///
    pub fn add_transaction(&mut self, transaction: T) -> &mut Self {
        self.transactions.push(transaction);

        self
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
fn test_blockchain_serde() {
    let mut blockchain = BlockChain::new(); // block #0 (genesis)

    // crating block #1
    let transaction = Transaction::new("s-1", "r-1", "message 1-1".to_string());
    blockchain.add_transaction(transaction);
    let transaction= Transaction::new("s-2", "r-2", "message 2-2".to_string() );
    blockchain.add_transaction(transaction);
    blockchain.create_next_block();

    // creating block #2
    let transaction = Transaction::new("s-1", "r-2", "message 1-2".to_string());
    blockchain.add_transaction(transaction);
    let transaction = Transaction::new("s-2", "r-1", "message 2-1".to_string());
    blockchain.add_transaction(transaction);
    blockchain.create_next_block();

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

    assert!(false);
}

//////////////////////////////// Tests ////////////////////////////

// about mining: https://bitcoin.stackexchange.com/a/7332

// RON + SERDE : https://pastebin.com/JKPBHbNK
