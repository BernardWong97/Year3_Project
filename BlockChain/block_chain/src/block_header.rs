use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::hashable::{
    clone_into_array, convert_u32_to_u8_array, convert_u64_to_u8_array, HashSha256, Hashable,
};

////////////////////////////// Block Header ////////////////////////////

//#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct BlockHeader {
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
            time_stamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
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
            index: self.index + 1,
            prev_hash: self.hash(),
            time_stamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            ..self.clone()
        }
    }

    ///
    /// Sets block difficulty to a given one.
    ///
    pub fn set_difficulty(&mut self, difficulty: usize) -> &mut Self {
        self.difficulty = difficulty;

        self
    }

    ///
    /// Sets block nonce to a given one.
    ///
    pub fn set_nonce(&mut self, nonce: usize) -> &mut Self {
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

//////////////////////////////// Tests /////////////////////////////////////////////////

#[test]
fn test_block_header_serde() {
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
fn test_block_header_mutators() {
    let mut header = BlockHeader::first();
    header.set_difficulty(5usize).set_nonce(5).increase_nonce();

    println!("{:?}", header);

    assert_eq!(header.index, 0);
    assert_eq!(header.nonce, 6);
    assert_eq!(header.difficulty, 5);
    assert_eq!(header.prev_hash, [0; 32]);

    //    assert!(false);
}

#[test]
fn test_block_header_hash() {
    let mut header = BlockHeader::first();
    println!("{:?}", header);

    let hash: HashSha256 = header.hash();
    header.increase_nonce();
    let hash1: HashSha256 = header.hash();
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
