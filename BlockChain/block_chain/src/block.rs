
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Sha512, Digest};
use std::fmt::Debug;

use crate::block_header::BlockHeader;
use crate::hashable::Hashable;
use crate::hashable::HashSha256;
use crate::hashable::clone_into_array;
use crate::transaction::Transaction;


////////////////////////////// Block Load ////////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockLoad<T>{
    //    p.ay: Money,
    data: Vec<T>,
}

#[allow(dead_code)]
impl <T>BlockLoad<T> {
    ///
    /// Create new block load
    ///
    pub fn new() -> Self {
        Self{ data: Vec::new(), }
    }

    ///
    /// Add load to block load
    ///
    pub fn add(&mut self, load:T) -> &mut Self where T: Hashable {
        self.data.push(load);

        self
    }
}

impl <T> Hashable for BlockLoad <T>
    where
        T: Hashable + Debug
{
    fn hash(&self) -> HashSha256{
        let mut hasher = Sha256::new();

        println!("{:?}", self.data);

        self.data.iter()
//            .inspect(|e| println!("{:?}", e.hash()))
            .for_each(|e| hasher.input(e.hash()));

        clone_into_array(hasher.result().as_slice())
    }
}

#[test]
fn test_block_load_serde(){
    let mut load = BlockLoad {
        data: Vec::new(),
    };

    load.data.push(String::from("I'm some load"));
    load.data.push(String::from("I'm to"));
    load.data.push(String::from("Same as I :)"));
    load.data.push(String::from(r#"Mess: "quotes \ /
    sd
    23rfasf34\\c\ \n"#));


    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&load).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a object.
    let deserialized: BlockLoad<String> = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);

    assert_eq!(deserialized.data, load.data);

//    assert!(false);
}

impl Hashable for String {
    fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.input(self.as_bytes());

        clone_into_array(hasher.result().as_slice())
    }
}

#[test]
fn test_block_load_add(){
    let mut load:BlockLoad<String> = BlockLoad::new();
    load.add(String::from("hello"))
        .add("bye-bye".to_string());


    println!("{:?}", load);

    assert_eq!(load.data[0], "hello".to_string());
    assert_eq!(load.data[1], "bye-bye".to_string());

//    assert!(false);
}

#[test]
fn test_block_load_hash(){
    let mut load:BlockLoad<String> = BlockLoad::new();
    load.add("one".to_string())
        .add("two".to_string())
        .add("three".to_string());

    let hash = load.hash();
    load.add("four".to_string());
    let hash1 = load.hash();

    println!("{:?}", load);
    println!("{:?}", hash);

    assert_ne!(hash, hash1);

//    assert!(false);
}

//////////////////////////////// Block ////////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Block{
    header: BlockHeader,
    load: BlockLoad<Transaction<String>>,
//    load: Vec<T>,
}

#[allow(dead_code)]
impl  Block{
    ///
    /// Creates new block
    ///
    pub fn genesis() -> Self {
        Self {
            header: BlockHeader::first(),
            load: BlockLoad::new(), }
    }

    pub fn next(&self) -> Self {
        Self{
            header: self.header.next(),
            load: BlockLoad::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction:Transaction<String>) -> &mut Self {
        self.load.data.push(transaction);

        self
    }

}

impl Hashable for Block {
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
    let mut block = Block::genesis();
    let mut transaction:Transaction<String> = Transaction::default();
    transaction.set_sender(String::from("s1"))
        .set_receiver(String::from("r1"))
        .set_amount(1usize)
        .set_load(String::from("load 1"));
    block.load.data.push(transaction);

    let mut transaction:Transaction<String> = Transaction::default();
    transaction.set_sender(String::from("s2"))
        .set_receiver(String::from("r2"))
        .set_amount(2usize)
        .set_load(String::from("load 2"));
    block.load.data.push(transaction);

    let mut transaction:Transaction<String> = Transaction::default();
    transaction.set_sender(String::from("s3"))
        .set_receiver(String::from("r3"))
        .set_amount(3usize)
        .set_load(String::from("load 3"));
    block.load.data.push(transaction);

    // Convert the Block to a JSON string.
    let serialized = serde_json::to_string(&block).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Block.
    let deserialized: Block = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);

    assert_eq!(deserialized.load.data, block.load.data);
    assert_eq!(deserialized.load.data[0], block.load.data[0]);
    assert_ne!(deserialized.load.data[1], block.load.data[2]);

//    assert!(false);
}