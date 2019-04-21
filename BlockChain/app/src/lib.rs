
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

use block_chain::BlockChain;
use block_chain::transaction::Transaction;
use node::Node;
use miner::Miner;
use crate::config::Config;


const CONFIG_FILE:&'static str = "settings_file.txt";


///////////// APP Error /////////////////////

#[derive(Debug)]
pub struct AppError{
    err: String,
}

impl AppError {
    pub fn new(err:String) -> Self {
        Self{ err }
    }
}

impl error::Error for AppError {
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node error is here!")
    }
}

impl convert::From<io::Error> for AppError {
    fn from(er: io::Error) -> Self {
        AppError::new(er.to_string())
    }
}



//////////////////////////// APP ///////////////////////////

//#[derive(Serialize, Deserialize, Debug)]
pub struct App <'a>{
    /// config:
    /// - blockchain_file,
    /// - node_settings,
    app_config: Config<'a>,
    chain: BlockChain<Transaction<String>>,
    node: Node<String>,
//    miner: Miner<'a>,
}

#[allow(dead_code)]
impl<'a> App<'a> {
    pub fn create(config:Config<'a>) -> Result<Self, AppError> {
//        if let Ok(conf) = Config::from(CONFIG_FILE) {
//
//        }

        let this = Self {
            app_config: config,
            chain: BlockChain::new(),
            node: Node::new(),
        };

        Ok(this)
    }

//    pub fn load() -> Result<Self, AppError> {
//        if let contents = fs::read_to_string(&config.filename){
//
//        }
//
//
//    }

    pub fn save(&self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    pub fn connect_to_network(&self) -> Result<(), Box<dyn error::Error>> {
        self.node.connect()
    }

    pub fn add_transaction(&mut self, transaction: Transaction<String>) -> Result<(), Box<dyn error::Error>> {
        self.chain.add_transaction(transaction);

        Ok(())
    }

    pub fn get_pending_transactions(&self) -> Result<&Vec<Transaction<String>>, Box<dyn error::Error>> {
        Ok(self.chain.get_pending_transactions())
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
