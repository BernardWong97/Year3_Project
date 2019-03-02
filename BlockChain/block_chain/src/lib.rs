//#![allow(non_snake_case)]
#![allow(unused_imports)]

/// Block submodule
///
/// # author: Mindaugas Sharskus
/// # date: 15-20-2019
///


mod block_header;
mod block;
pub mod hashable;
pub mod transaction;
pub mod message;


use serde::{Serialize, Deserialize};
use sha2::{Sha256, Sha512, Digest};
use std::mem;
use std::convert::AsMut;
use uuid::Uuid;

use crate::transaction::Transaction;
use crate::block::Block;


//////////////////////////////// Block Chain ////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockChain {
    chain: Vec<Block>,
    uuid:Uuid,
    //    chain: BTreeSet<Block>,     // ordered set, blocks should be unique anyway
    transactions: Vec<Transaction<String>>, // pending transactions
}

#[allow(dead_code)]
impl BlockChain {
    // TODO: Implement merkle tree functionality fo transaction confirmation.
    ///
    /// Creates new blockchain with genesis block in it
    ///
    pub fn new() -> Self {
        let mut chain = Vec::new();     // create chain
        chain.push(Block::genesis());        // add genesis block to chain

        Self{
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
    pub fn create_next_block(&mut self) -> &Block {
        let mut block = self.chain.last()
            .unwrap_or_else(|| {
                panic!("Here is no blocks in blockchain");
            }).next();  // create new "next" block


        // add all pending transactions to new block //

        // https://doc.rust-lang.org/error-index.html#E0507
        // second part(close to end)
        mem::replace(&mut self.transactions, Vec::new())
            .into_iter()
            .for_each(|tr|{block.add_transaction(tr);});

        assert_eq!(self.transactions.len(), 0);



        // add new block to blockchain
        self.chain.push(block);

        // return blockchain last block reference
        &self.chain.last().unwrap_or_else(||
            panic!("Here is no blocks in blockchain")
        )
    }

    ///
    /// Add transaction to pending transactions
    ///
    pub fn add_transaction(&mut self, transaction:Transaction<String>) -> &mut Self {
        self.transactions.push(transaction);

        self
    }
}


//////////////////////////////// Tests /////////////////////////////////////////////////

#[test]
fn test_blockchain_serde(){
    let mut blockchain = BlockChain::new(); // block #0 (genesis)

    // crating block #1
    let mut transaction:Transaction<String> = Transaction::default();
    transaction.set_sender(String::from("s1"))
        .set_receiver(String::from("r1"))
        .set_amount(1usize)
        .set_load(String::from("load 1"));
    blockchain.add_transaction(transaction);

    let mut transaction:Transaction<String> = Transaction::default();   // shadowing variable
    transaction.set_sender(String::from("s2"))
        .set_receiver(String::from("r2"))
        .set_amount(2usize)
        .set_load(String::from("load 2"));
    blockchain.add_transaction(transaction);

    blockchain.create_next_block();

    // creating block #2
    let mut transaction:Transaction<String> = Transaction::default();
    transaction.set_sender(String::from("s11"))
        .set_receiver(String::from("r11"))
        .set_amount(11usize)
        .set_load(String::from("load 11"));
    blockchain.add_transaction(transaction);

    let mut transaction:Transaction<String> = Transaction::default();   // shadowing variable
    transaction.set_sender(String::from("s12"))
        .set_receiver(String::from("r12"))
        .set_amount(12usize)
        .set_load(String::from("load 12"));
    blockchain.add_transaction(transaction);

    blockchain.create_next_block();



    // Convert the Block to a JSON string.
    let serialized = serde_json::to_string(&blockchain).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Block.
    let deserialized: BlockChain = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    assert_eq!(deserialized.chain.len(), blockchain.chain.len());
    assert_eq!(deserialized.chain[1], blockchain.chain[1]);
    assert_ne!(deserialized.chain[0], blockchain.chain[1]);

    println!("{:?}", blockchain);

//    assert!(false);
}



//////////////////////////////// Tests ////////////////////////////

// about mining: https://bitcoin.stackexchange.com/a/7332


