
//!
//! App controller are controlling BlockChain, Node and Miner.
//! AppController is controlled by API.
//!
//! Author: Mindaugas Sharskus
//! Date: 31-03-2019
//!


use block_chain::BlockChain;
use block_chain::transaction::Transaction;


struct AppController <'a>{
    chain: BlockChain<Transaction<'a, String>>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
