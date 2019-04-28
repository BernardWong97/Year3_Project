//!
//! `Block` is critical part of `BlockChain`
//!
//! # author: Mindaugas Sharskus
//! # date: 15-02-2019
//!
//! ToDo:
//! - find better name for method `add_record`.
//! - ?? hide `BlockHeader` ??
//! - add blockchain uuid as parameter to represent ad blocks id

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use core::borrow::BorrowMut;
use uuid::Uuid;

use crate::block_header::BlockHeader;
use crate::{hashable, BlockChain};
use crate::hashable::HashSha256;
use crate::hashable::Hashable;
use crate::transaction::Transaction;


//////////////////////////////// Block ////////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Block<T> {
//    chain_uuid: Uuid,
    pub header: BlockHeader,
    pub load: Vec<T>,
}

#[allow(dead_code)]
impl<T> Block<T>
where
    T: Hashable
{
    /// Creates new block
    pub fn genesis() -> Self {
        Self {
            header: BlockHeader::first(),
            load: Vec::new(),
        }
    }

    /// Creates new block. Uses `self` as reference for block creation.
    pub fn next(&self) -> Self {
        Self {
            header: self.header.next(self.hash()),
            load: Vec::new(),
        }
    }

    /// Adds single record to blocks load.
    pub fn add_record(&mut self, record: T) -> &mut Self {
        self.load.push(record);

        self
    }

    /// Get previous block hash
    pub fn get_prev_hash(&self) -> &HashSha256 {
        &self.header.get_previous_hash()
    }

    /// Get block index
    pub fn get_index(&self) -> usize {
        self.header.index
    }
}

impl<T> Hashable for Vec<T>
where
    T: Hashable,
{
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        self.iter().for_each(|e| hasher.input(e.hash()));

        hashable::clone_into_array(hasher.result().as_slice())
    }
}

impl<T> Hashable for Block<T>
where
    T: Hashable,
{
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(self.header.hash());
        hasher.input(self.load.hash());

        hashable::clone_into_array(hasher.result().as_slice())
    }
}

//////////////////////////////// Tests /////////////////////////////////////////////////

#[test]
fn test_block_serde() {
    let test = vec![
        "🐰",
        "Happy",
        "Easter",
        "🐰",
    ];
    let mut block: Block<String> = Block::genesis();
    test.iter().for_each(|t|{
        block.add_record(t.to_string());
    });

    // Convert the Block to a JSON string.
    let serialized = serde_json::to_string(&block).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Block.
    let deserialized: Block<String> =
        serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);

    assert_eq!(deserialized.load, block.load);
    assert_eq!(deserialized.load[0], block.load[0]);
    assert_ne!(deserialized.load[1], block.load[2]);

    assert!(false);
}
