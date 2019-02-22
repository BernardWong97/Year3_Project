//#![allow(non_snake_case)]
#![allow(unused_imports)]

/// Block submodule
///
/// # author: Mindaugas Sharskus
/// # date: 15-20-2019
///



use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Sha512, Digest};
use serde::{Serialize, Deserialize};





trait Hashable <T> {
    fn hash(&self) -> Vec<u8>;
}



////////////////////////////// Block Header ////////////////////////////

//#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct BlockHeader {
    index: u32,
    prev_hash: Vec<u8>,
    time_stamp: u64,
//    difficulty: usize,
//    nonce: usize,
//    merkle: BlockHash,    TODO: do it later if we have time for it
//    blockchain_uuid:
}

//#[allow(non_snake_case)]
#[allow(dead_code)]
impl BlockHeader {
    pub fn new(index:u32, prev_hash:Vec<u8>) -> Self {
        Self {
            index,
            prev_hash,
            time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
//            difficulty,
//            nonce,
        }
    }

}

//impl Hashable for BlockHeader {
//    fn hash(&self) -> Vec<u8> {
//        let mut hasher = Sha256::new();
//        hasher.input(self.index);
//        hasher.result()
//    }
//}

#[test]
fn test_block_header_serde(){
//    let BlockHeader::new(0u32, )
    let mut hasher = Sha256::new();
    hasher.input(b"0000");
    let prev_hash = hasher.result().to_vec();

    let header = BlockHeader {
        index: 0,
        prev_hash,
        time_stamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    };

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

////////////////////////////// Block Load ////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
struct BlockLoad<T>{
//    pay: Money,
    data: Vec<T>,
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


//////////////////////////////// Block ////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    header: BlockHeader,
    load: BlockLoad<String>,
}

#[allow(dead_code)]
impl Block {
    pub fn next(&self, load:BlockLoad<String>) -> Self {
        Self {
            header: BlockHeader::new(
                self.header.index +1,
                self.header.prev_hash.clone(),
//                self.header.difficulty,
//                self.header.nonce,
            ),
            load,
        }
    }

//    pub fn get_hash(&self) -> BlockHash {
//
//    }

}

#[test]
fn test_block_serde(){
    let mut block = Block {
        header: BlockHeader {
            index: 0,
            prev_hash: Vec::new(),
            time_stamp: 0
        },
        load: BlockLoad {
            data: Vec::new(),
        },
    };

    block.load.data.push(String::from("I'm some load"));
    block.load.data.push(String::from("I'm to"));
    block.load.data.push(String::from("Same as I :)"));
    block.load.data.push(String::from(r#"Mess: "quotes \ /
    sd
    23rfasf34\\c\ \n"#));


    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&block).unwrap();
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a object.
    let deserialized: Block = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);

    assert_eq!(deserialized.load.data, block.load.data);

//    assert!(false);
}



//////////////////////////////// Block Chain ////////////////////////////

struct BlockChain {
    chain: Vec<Block>
}

impl BlockChain {

}




//////////////////////////////// Tests ////////////////////////////
//
