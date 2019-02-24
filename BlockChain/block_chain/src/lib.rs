//#![allow(non_snake_case)]
#![allow(unused_imports)]

/// Block submodule
///
/// # author: Mindaugas Sharskus
/// # date: 15-20-2019
///



use std::time::{SystemTime, UNIX_EPOCH};
//use std::convert::TryInto;
use std::convert::AsMut;

use sha2::{Sha256, Sha512, Digest};
use serde::{Serialize, Deserialize};
use std::collections::BTreeSet;
use std::fmt::Debug;


////////////////////////////// Hash type ////////////////////////////

const HASH_BYTE_SIZE: usize = 32;

type HashSha256 = [u8; HASH_BYTE_SIZE];


////////////////////////////// Hashable trait ////////////////////////////

trait Hashable {
    fn hash(&self) -> HashSha256;
}



////////////////////////////// Block Header ////////////////////////////

//#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
struct BlockHeader {
    // More at: https://bitcoin.stackexchange.com/a/7332
    index: u32,
    prev_hash: HashSha256,
    time_stamp: u64,
    difficulty: usize,
    nonce: usize,
// https://sitano.github.io/merkle_light/merkle_light/index.html#modules
//    merkle: BlockHash,    TODO: do it later if we have time for it
//    blockchain_uuid:
}

//#[allow(non_snake_case)]
#[allow(dead_code)]
impl BlockHeader {
    ///
    /// Create new block header
    ///
    pub fn first() -> Self {
        Self {
            index: 0u32,
            prev_hash: HashSha256::default(),
            time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
            difficulty: 0usize,
            nonce: 0usize,
        }
    }

    ///
    /// ## Create new next block header.
    /// New block header will have:
    ///     - index +1
    ///     - time_stamp: now()
    ///     - prev_hash: set to current(self) block headers hash
    ///     - .. all rest same as current block header
    ///
    pub fn next(&self) -> Self {
        Self {
            index: self.index +1,
            prev_hash: self.hash(),
            time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
            ..self.clone()
        }
    }

    ///
    /// Sets block difficulty to a given one.
    ///
    pub fn set_difficulty(&mut self, difficulty:usize) -> &mut Self {
        self.difficulty = difficulty;

        self
    }

    ///
    /// Sets block nonce to a given one.
    ///
    pub fn set_nonce(&mut self, nonce:usize) -> &mut Self {
        self.nonce = nonce;

        self
    }

    ///
    /// Increase block nonce by 1 .
    ///
    pub fn increase_nonce(&mut self) -> &mut Self {
        self.nonce += 1usize;

        self
    }

}

impl Hashable for BlockHeader {
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(convert_u32_to_u8_array(self.index));
        hasher.input(convert_u64_to_u8_array(self.difficulty as u64));
        hasher.input(self.prev_hash);
        hasher.input(convert_u64_to_u8_array(self.time_stamp));
        hasher.input(convert_u64_to_u8_array(self.nonce as u64));

        clone_into_array(hasher.result().as_slice())
    }
}

#[test]
fn test_block_header_serde(){
    let header = BlockHeader::first();

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&header).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a object.
    let deserialized: BlockHeader = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    assert_eq!(deserialized.index, header.index);
    assert_eq!(deserialized.prev_hash, header.prev_hash);
    assert_eq!(deserialized.time_stamp, header.time_stamp);

//    assert!(false);
}

#[test]
fn test_block_header_mutators(){
    let mut header = BlockHeader::first();
    header.set_difficulty(5usize)
        .set_nonce(5)
        .increase_nonce();

    println!("{:?}", header);

    assert_eq!(header.index, 0);
    assert_eq!(header.nonce, 6);
    assert_eq!(header.difficulty, 5);
    assert_eq!(header.prev_hash, [0; 32]);

//    assert!(false);
}

#[test]
fn test_block_header_hash(){
    let mut header = BlockHeader::first();
    println!("{:?}", header);

    let hash:HashSha256 = header.hash();
    header.increase_nonce();
    let hash1:HashSha256 = header.hash();
    println!("{:?}", hash);
    println!("{:?}", header);

    assert_ne!(hash, hash1);


    let header1 = header.next();
    let header2 = header1.next();
    println!("{:?}", header1);
    println!("{:?}", header1.hash());
    println!("{:?}", header2);
    println!("{:?}", header2.hash());

    assert_ne!(header1.hash(), header2.hash());

//    assert!(false);
}


//////////////////////////////// Transaction ///////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Transaction <D> {
    sender: String,
    receiver: String,
    amount: usize,
    load: D,
}

#[allow(dead_code)]
impl <D>Transaction<D>
    where
        D: Hashable
{
    pub fn set_sender(&mut self, sender:String) -> &mut Self {
        self.sender = sender;

        self
    }

    pub fn set_receiver(&mut self, receiver:String) -> &mut Self {
        self.receiver = receiver;

        self
    }

    pub fn set_amount(&mut self, amount:usize) -> &mut Self {
        self.amount = amount;

        self
    }

    pub fn set_load(&mut self, load:D) -> &mut Self {
        self.load = load;

        self
    }

}

impl <D>Default for Transaction<D>
    where
        D: Default
{
    fn default() -> Self {
        Self {
            sender: String::new(),
            receiver: String::new(),
            amount: 0usize,
            load: D::default(),
        }
    }
}

impl <D>Hashable for Transaction <D>
    where
        D: Hashable
{
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(self.sender.as_str());
        hasher.input(self.receiver.as_str());
        hasher.input(convert_u64_to_u8_array(self.amount as u64));
        hasher.input(self.load.hash());

        clone_into_array(hasher.result().as_slice())
    }
}





////////////////////////////// Block Load ////////////////////////////

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BlockLoad<T>{
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
struct Block{
    header: BlockHeader,
    load: BlockLoad<Transaction<String>>,
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

}

impl Hashable for Block {
    fn hash(&self) -> HashSha256 {
        let mut hasher = Sha256::new();
        hasher.input(self.header.hash());
        hasher.input(self.load.hash());

        clone_into_array(hasher.result().as_slice())
    }
}


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



//////////////////////////////// Block Chain ////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
struct BlockChain {
//    uuid:         // TODO: Create unique ID to identify blockchain. (...? use hashed current time??)
    chain: Vec<Block>,
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
            chain,
            transactions: Vec::new(),
        }
    }

    ///
    /// Creates new "next" block.
    /// New block will have all pending transactions.
    ///
    pub fn create_next_block(&mut self) -> &Block {
        let mut block = self.chain.last()
            .unwrap_or_else(|| {
                panic!("Here is no blocks in blockchain");
            }).next();  // create new "next" block

        // add all pending transactions to new block
        block.load.data.append(&mut self.transactions);
        // clear pending transactions
        self.transactions.clear();

        // add new block to blockchain
        self.chain.push(block);

        // return blockchain last block reference
        &self.chain.last().unwrap_or_else(||
            panic!("Here is no blocks in blockchain")
        )
    }

    ///
    /// Add transaction to pending transactions
    pub fn add_transaction(&mut self, transaction:Transaction<String>) -> &mut Self {
        self.transactions.push(transaction);

        self
    }
}

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


//////////////////////////////// Helper functions ////////////////////////////

///
///  <br>[code reference](https://stackoverflow.com/a/37679019/5322506)
///
fn clone_into_array<A, T>(slice: &[T]) -> A
    where
        A: Default + AsMut<[T]>,
        T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

///
/// Concerts `u32` number as `u8` array
/// <br>[code reference](https://freestartupkits.com/articles/technology/cryptocurrency-news-and-tips/ultimate-rust-blockchain-tutorial/)
///
pub fn convert_u32_to_u8_array(val: u32) -> [u8; 4] {
    return [
        val as u8,
        (val >> 8 * 1) as u8,
        (val >> 8 * 2) as u8,
        (val >> 8 * 3) as u8,
    ]
}

///
/// Concerts `u32` number as `u8` array
/// <br>[code reference](https://freestartupkits.com/articles/technology/cryptocurrency-news-and-tips/ultimate-rust-blockchain-tutorial/)
///
pub fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
    return [
        val as u8,
        (val >> 8 * 1) as u8,
        (val >> 8 * 2) as u8,
        (val >> 8 * 3) as u8,
        (val >> 8 * 4) as u8,
        (val >> 8 * 5) as u8,
        (val >> 8 * 6) as u8,
        (val >> 8 * 7) as u8,
    ]
}