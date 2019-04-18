
//!
//! App app are controlling BlockChain, Node and Miner.
//! AppController is controlled by API.
//!
//! Author: Mindaugas Sharskus
//! Date: 31-03-2019
//!


mod config;

use std::error::Error;
use std::fs;

use serde::{Deserialize, Serialize, Serializer};

use block_chain::BlockChain;
use block_chain::transaction::Transaction;
use node::Node;
use miner::Miner;




//#[derive(Serialize, Deserialize, Debug)]
pub struct App <'a>{
    chain: BlockChain<Transaction<'a, String>>,
    node: Node<String>,
//    miner: Miner<'a>,
}

#[allow(dead_code)]
impl<'a> App<'a> {
    pub fn build() -> Self {
//        if let Ok(mut contents) = fs::read_to_string(&config.filename){
//
//        }
//        else{
//
//        }

        Self {
            chain: BlockChain::new(),
            node: Node::new(),
        }
    }

//    pub fn load() -> Result<Self, Box<dyn Error>> {
//        if let contents = fs::read_to_string(&config.filename){
//
//        }
//
//
//    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn connect_to_network(&self) -> Result<(), Box<dyn Error>> {
        self.node.connect()
    }

    pub fn add_transaction(&mut self, transaction: Transaction<'a, String>) -> Result<(), Box<dyn Error>> {
        self.chain.add_transaction(transaction);

        Ok(())
    }


}


#[cfg(test)]
mod tests {
//    use crate::{AppConfig, App};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

//    #[test]
//    fn test_config(){
//        let config = AppConfig{
//            settings_file: "",
//            chain_file: "",
//            miner: false
//        };
//
//        let app = App::new(config);
//
//        assert_eq!("", app.config.settings_file, "Should be empty");
//        assert_eq!("", app.config.chain_file, "Should be empty");
//        assert!(!app.config.miner);
//    }
}
