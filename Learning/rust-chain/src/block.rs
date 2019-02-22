use std::time::SystemTime;
use chrono::prelude::*;

const HASH_BYTE_SIZE: usize = 32;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Debug)]
pub struct Block{
    time_stamp: SystemTime,
    prev_block_hash: Sha256Hash,
    data: Vec<u8>,
}
impl Block{
    // Constructor
    pub fn new(data:&str, prev_block_hash: Sha256Hash) -> Self {
        Self {
            time_stamp: SystemTime::now(),
            prev_block_hash,
            data,
//            data = data.as_bytes().to_owned(),
        }
    }

    pub fn calculate_hash(&self){
        println!("Hello");
        println!("{:?}", self.data);
    }
}