//!
//! Block miner
//!
//! Author: Mindaugas Sharskus
//! date: 31-03-2019
//!
//! Reference:
//!     Lifetimes:
//!         https://doc.rust-lang.org/1.30.0/book/second-edition/ch19-02-advanced-lifetimes.html
//!

use std::thread;

use sha2;
use block_chain::Block;
use block_chain::hashable::{Hashable, HashSha256};
use block_chain::hashable;

#[derive(Debug)]
struct Miner<'a, T: 'a>{
    block: &'a mut Block<T>,
}

impl<'a, T> Miner<'a, T>
where
    T: Hashable
{
    pub fn new(block: &'a mut Block<T>) -> Self {
        Self{ block }
    }

    pub fn run(&mut self) -> usize {
        loop {
            let hash = self.block.header.hash();
//            println!("{:?}", &hash);

            let slice = &hash[..self.block.header.difficulty];
            let num = slice
                .iter()
                .filter(|e| *e != &0x0u8)
                .count();

            if num == 0 {
                let nonce = self.block.header.nonce;
                println!("Found {:?}", nonce);
                println!("{:?}", &hash);

                break nonce;
            }

            self.block.header.increase_nonce();
        }

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

        let miner = Miner::new(&mut block);

        println!("{:?}", miner);

        assert!(false);
    }

    #[test]
    fn test_mining(){
        let mut block = Block::genesis();
        let mut block = block.next();
        block.header.set_difficulty(2usize);
        block.add_record(String::from("hi"));
        block.add_record(String::from("Rust"));


//        for x in 0..10 {
//            let hash = block.header.hash();
//            let slice = &hash[..block.header.difficulty];
//            println!("{:?}", slice);

//            let mut miner = Miner::new(&mut block);
//            miner.run();

//            println!("{:?}", miner);
//        }




//        let miner = Miner::new(&mut block);
//        miner.run();

//        println!("{:?}", miner);



        let child = thread::spawn(move || {
            let mut miner = Miner::new(&mut block);
            miner.run()
        });


        let res = child.join();

        println!("{:?}", res);
//        println!("{:?}", &block);

        assert!(false);


    }
}
