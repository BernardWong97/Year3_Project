//!
//! App app are controlling BlockChain, Node and Miner.
//! AppController is controlled by API.
//!
//! # author: Mindaugas Sharskus
//! # date: 31-03-2019
//!


pub mod config;

use std::error;
use std::fs;
use std::fmt;
use std::io;
use std::convert;

use core::fmt::Display;

use serde::{Deserialize, Serialize, Serializer};

use block_chain::{BlockChain, Block};
use block_chain::transaction::Transaction;
use node::Node;
use miner::Miner;
use crate::config::Config;


const CONFIG_FILE:&'static str = "settings_file.txt";

pub const KEY_APP_USER:&'static str = "app_user";
pub const KEY_MINER_URL:&'static str = "miners_url";

//////////////////////////// APP ///////////////////////////

pub type Message = Transaction<String>;

//#[derive(Serialize, Deserialize, Debug)]
pub struct App <'a>{
    /// config:
    /// - blockchain_file,
    /// - node_settings,
    pub config: Config<'a>,
    blockchain: BlockChain<Transaction<String>>,
    node: Node<String>,
//    miner: Miner<'a>,
}

#[allow(dead_code)]
impl<'a> App<'a> {
    pub fn create(config:Config<'a>) -> Result<Self, Box<dyn error::Error>> {
        let this = Self {
            config: config,
            blockchain: BlockChain::new(),
            node: Node::new(),
        };

        Ok(this)
    }

    pub fn save(&self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    pub fn connect_to_network(&self) -> Result<(), Box<dyn error::Error>> {
        self.node.connect()
    }

    pub fn add_transaction(&mut self, transaction: Message) -> Result<(), Box<dyn error::Error>> {
        self.blockchain.add_transaction(transaction);

        Ok(())
    }

    pub fn get_pending_transactions(&self) -> Result<&Vec<Message>, Box<dyn error::Error>> {
        Ok(self.blockchain.get_pending_transactions())
    }

    pub fn get_genesis_block(&self) -> Result<&Block<Message>, Box<dyn error::Error>> {
        Ok(self.blockchain.get_block_genesis())
    }

    pub fn get_last_block(&self) -> Result<&Block<Message>, Box<dyn error::Error>> {
        Ok(self.blockchain.get_block_last())
    }

    pub fn add_block(&mut self, block: Block<Message>) -> Result<(), Box<dyn error::Error>> {
        self.blockchain.add_block(block);

        Ok(())
    }

    pub fn get_blocks_from(&self, block: &Block<Message>) -> Result<&[Block<Message>], Box<dyn error::Error>> {
        self.blockchain.get_blocks_from(block)
    }



}

////////////////////// Tests ////////////////////////

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
