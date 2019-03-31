use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::fmt::Debug;

use crate::block_header::BlockHeader;
use crate::hashable;
use crate::hashable::HashSha256;
use crate::hashable::Hashable;
use crate::transaction::Transaction;

//////////////////////////////// Block ////////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Block<T> {
    pub header: BlockHeader,
    //    load: BlockLoad<Transaction<String>>,
    load: Vec<T>,
}

#[allow(dead_code)]
impl<T> Block<T>
where
    T: Hashable
{
    ///
    /// Creates new block
    ///
    pub fn genesis() -> Self {
        Self {
            header: BlockHeader::first(),
            load: Vec::new(),
        }
    }

    ///
    /// Creates new block. Uses `self` as reference for block creation.
    ///
    pub fn next(&self) -> Self {
        Self {
            header: self.header.next(),
            load: Vec::new(),
        }
    }

    ///
    /// Adds single record to blocks load.
    ///
    pub fn add_record(&mut self, record: T) -> &mut Self {
        self.load.push(record);

        self
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
    let mut block: Block<Transaction<String>> = Block::genesis();
    let transaction = Transaction::new("s-1", "r-1", "message 1-1".to_string());
    block.load.push(transaction);
    let transaction = Transaction::new("s-2", "r-2", "message 2-2".to_string());;
    block.load.push(transaction);
    let transaction = Transaction::new("s-3", "r-3", "message 3-3".to_string());
    block.load.push(transaction);

    // Convert the Block to a JSON string.
    let serialized = serde_json::to_string(&block).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Block.
    let deserialized: Block<Transaction<String>> =
        serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);

    assert_eq!(deserialized.load, block.load);
    assert_eq!(deserialized.load[0], block.load[0]);
    assert_ne!(deserialized.load[1], block.load[2]);

    //    assert!(false);
}
