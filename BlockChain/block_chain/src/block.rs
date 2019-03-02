
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Sha512, Digest};
use std::fmt::Debug;

use crate::block_header::BlockHeader;
use crate::hashable::Hashable;
use crate::hashable::HashSha256;
use crate::hashable::clone_into_array;
use crate::transaction::Transaction;



//////////////////////////////// Block ////////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Block<T>
{
    pub header: BlockHeader,
//    load: BlockLoad<Transaction<String>>,
    load: Vec<T>,
}

#[allow(dead_code)]
impl  <T>Block<T>
where
    T: Hashable + Default,
{
    ///
    /// Creates new block
    ///
    pub fn genesis() -> Self {
        Self {
            header: BlockHeader::first(),
            load: Vec::new(), }
    }

    ///
    /// Creates new block. Uses `self` as reference for block creation.
    ///
    pub fn next(&self) -> Self {
        Self{
            header: self.header.next(),
            load: Vec::new(),
        }
    }

    ///
    /// Adds single record to blocks load.
    ///
    pub fn add_record(&mut self, record:T) -> &mut Self {
        self.load.push(record);

        self
    }

}

impl <T>Hashable for Vec<T>
where
    T: Hashable,
{
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        self.iter()
            .for_each(|e|hasher.input(e.hash()));

        clone_into_array(hasher.result().as_slice())
    }
}

impl <T>Hashable for Block<T>
    where
        T: Hashable + Default,
{
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(self.header.hash());
        hasher.input(self.load.hash());

        clone_into_array(hasher.result().as_slice())
    }
}


//////////////////////////////// Tests /////////////////////////////////////////////////

#[test]
fn test_block_serde(){
    let mut block:Block<Transaction<String, String>> = Block::genesis();
    let mut transaction:Transaction<String, String> = Transaction::default();
    transaction.add_sender(String::from("s1"))
        .add_receiver(String::from("r1"))
        .add_value(String::from("Some value"))
        .add_load(String::from("load 1"));
    block.load.push(transaction);

    let mut transaction:Transaction<String, String> = Transaction::default();
    transaction.add_sender(String::from("s2"))
        .add_receiver(String::from("r2"))
        .add_value(String::from("Some value"))
        .add_load(String::from("load 2"));
    block.load.push(transaction);

    let mut transaction:Transaction<String, String> = Transaction::default();
    transaction.add_sender(String::from("s3"))
        .add_receiver(String::from("r3"))
        .add_value(String::from("Some value"))
        .add_load(String::from("load 3"));
    block.load.push(transaction);

    // Convert the Block to a JSON string.
    let serialized = serde_json::to_string(&block).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Block.
    let deserialized: Block<Transaction<String, String>> = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);

    assert_eq!(deserialized.load, block.load);
    assert_eq!(deserialized.load[0], block.load[0]);
    assert_ne!(deserialized.load[1], block.load[2]);

//    assert!(false);
}

