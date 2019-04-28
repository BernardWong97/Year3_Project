//!
//! Block miner
//!
//! # author: Mindaugas Sharskus
//! # date: 31-03-2019
//!
//! Reference:
//!     Lifetimes:
//!         https://doc.rust-lang.org/1.30.0/book/second-edition/ch19-02-advanced-lifetimes.html
//!
//! Todo: (enchantments)
//! - create multi threaded miner.

use std::thread;

use sha2;
use block_chain::block_header::BlockHeader;
use block_chain::hashable::{Hashable, HashSha256};
use block_chain::hashable;
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Miner<'a>{
    header: &'a BlockHeader,
}

#[allow(dead_code)]
impl<'a> Miner<'a> {
    ///
    /// create new miner
    ///
    pub fn new(header: &'a BlockHeader) -> Self {
        Self{ header }
    }

    ///
    /// start mining (current thread)
    ///
    pub fn start(&self) -> usize {
        let mut tmp = self.header.clone();

        loop {
            let hash = tmp.hash();

            let slice = &hash[..tmp.difficulty];
            let num = slice
                .iter()
                .filter(|e| *e != &0u8)
                .count();

            if num == 0 {
                let nonce = tmp.nonce;

                break nonce;
            }

            tmp.increase_nonce();
        }
    }

    ///
    /// Start mining on new thread
    ///
    pub fn start_thread(&self) -> JoinHandle<usize> {
        let mut header = self.header.clone();

        thread::spawn(move || {
            loop {
                let hash = header.hash();
                let slice = &hash[..header.difficulty];

                let n = slice
                    .iter()
                    .filter(|e| *e != &0u8)
                    .count();

                if n == 0 {
                    break header.nonce;
                }

                header.increase_nonce();
            }
        })
    }
}



#[cfg(test)]
mod tests {
    use block_chain::Block;
    use crate::Miner;
    use std::thread;
    use block_chain::hashable::Hashable;

    #[test]
    fn test_instance() {
        let mut block = Block::genesis();
        let mut block = block.next();
        block.add_record(String::from("genesis"));
        block.add_record(String::from("genesis1"));

        let miner = Miner::new(& block.header);

        println!("{:?}", miner);

//        assert!(false);
    }

    #[test]
    fn test_mining(){
        let mut block = Block::genesis();
        let mut block = block.next();
        block.header.set_difficulty(2usize);
        block.add_record(String::from("hi"));
        block.add_record(String::from("Rust"));

        // copy header ..
        let header = block.header.clone();
        // .. and pass it to the thread
        let child = thread::spawn(move || {
            let mut miner = Miner::new(& header);
            miner.start()
        });

        // Wait for tread to finish and take result (nonce)
        let res = child.join();

        println!("{:?}", res);
        println!("{:?}", &block.header);

        println!("hash before: {:?}", &block.header.hash());

        // update original header with a given value
        block.header.set_nonce(res.unwrap());
        println!("{:?}", &block.header);
        println!("hash after: {:?}", &block.header.hash());

//        assert!(false);
    }

    #[test]
    fn test_threaded_mining() {
        let mut block = Block::genesis();
        let mut block = block.next();
        block.header.set_difficulty(2usize);
        block.add_record(String::from("hi"));
        block.add_record(String::from("Rust"));

        let miner = Miner::new(&block.header);
        let handle = miner.start_thread();
        let result = handle.join().unwrap();

        println!("nonce: {:?}", result);

        block.header.set_nonce(result);
        let new_hash = block.header.hash();
        let new_slice = &new_hash[..block.header.difficulty].to_vec();
        let expected = vec![0u8; block.header.difficulty];

        assert_eq!(new_slice, &expected);

//        assert!(false);
    }
}
