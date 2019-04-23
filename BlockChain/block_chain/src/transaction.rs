//!
//! `Transaction` is `Block`s load.
//!
//! # author: Mindaugas Sharskus
//! # date: 20-02-2019
//!
//! ToDo:
//! - add block hash to `TransactionID` to gain extra security (extras).
//! - rename `::new` to `::add` or similar to more reflect what it does.
//!

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::hashable::clone_into_array;
use crate::hashable::convert_u64_to_u8_array;
use crate::hashable::HashSha256;
use crate::hashable::Hashable;
use crate::block_header::BlockHeader;
use crate::{Block, BlockChain};
use core::borrow::BorrowMut;
use std::fmt::Debug;

//////////////////////////////// Transaction ///////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TransactionID {
    timestamp: u64,
    blockchain_uuid: Uuid,
    // block_hash: HashSha256,     // hash of the block then transaction was generated
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Transaction<T> {
    pub id: TransactionID,
    pub sender: String,
    pub receiver: String,
    pub value: usize,
    pub load: T,    // in our case it will be message
}

#[allow(dead_code)]
impl<T> Transaction<T>
where
    T: Hashable + Debug,
{
    pub fn new(sender: &str, receiver: &str, load: T, blockchain: &mut BlockChain<Transaction<T>>) {
        // create transaction
        let transaction = Self {
            id: TransactionID {
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
                blockchain_uuid: blockchain.get_uuid().clone(),
            },
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            value: 0,
            load,
        };
        // add transaction to the Blockchain
        blockchain.add_transaction(transaction);
    }
}



impl<T> Hashable for Transaction<T>
where
    T: Hashable,
{
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(&self.sender);
        hasher.input(&self.receiver);
        hasher.input(convert_u64_to_u8_array(self.value as u64));
        hasher.input(self.load.hash());

        clone_into_array(hasher.result().as_slice())
    }
}

//////////////////////////////// Tests /////////////////////////////////////////////////

#[test]
fn test_transaction_serde() {
    let mut blockchain = BlockChain::new();
    // add transaction to the block
    Transaction::new(
        "s-1", "r-1",
        "message 1-1".to_string(),
        blockchain.borrow_mut(),
    );
    let transactions = blockchain.get_pending_transactions();
    let tr = &transactions[0];
    // serialized
    let ser = serde_json::to_string(tr).unwrap();
    println!("Serialized = {}", ser);
    //deserialized
    let de:Transaction<String> = serde_json::from_str(&ser).unwrap();
    println!("Deserialized = {:?}", de);

    assert!(false);
}