//!
//! App app are controlling BlockChain, Node and Miner.
//! AppController is controlled by API.
//!
//! # author: Mindaugas Sharskus
//! # date: 31-03-2019
//!
//! ToDo (enchantments):
//! - append changes to file instead overwriting it each time blockchain is saved.
//! - implement floating points for the transaction (message) cost
//! - Add ability to send crypto credits with the message


pub mod config;
mod error;

use crate::config::Config;
pub use crate::error::AppError;

use std::error::Error;
use std::fs;
use std::fmt;
use std::io;
use std::convert;

use std::io::{Write, Read};

use core::fmt::Display;

use serde::{Deserialize, Serialize, Serializer};
use block_chain::{BlockChain, Block};
use block_chain::transaction::{Transaction, TransactionID};
use node::Node;
use miner::Miner;
use core::borrow::Borrow;
use uuid::Uuid;

pub const CONFIG_FILE:&'static str = "settings_file.txt";

pub const KEY_APP_USER:&'static str = "app_user";
pub const KEY_MINER_URL:&'static str = "miners_url";
pub const KEY_BLOCKCHAIN_FILE:&'static str = "blockchain_file";
pub const BLOCK_REWARD: usize = 20;
pub const MESSAGE_RECEIVE_REWARD: usize = 2usize;
pub const MESSAGE_SEND_COST: usize = 7usize;



//////////////////////////// APP ///////////////////////////

pub type Message = Transaction<String>;

#[derive(Deserialize, Debug)]
pub struct MessageTemplate {
    pub receiver: String,
    pub value: usize,
    pub text: String,
}

#[derive(Serialize, Debug)]
pub struct AppInfo<'i> {
    username: &'i str,
    user_balance: i64,  // balance only whole numbers (no fractions)
    blockchain_uuid: &'i Uuid,
    chain_len: usize,
    block_reward: usize,
    message_read_reward: usize,
    message_send_cost: usize,
}

//#[derive(Serialize, Deserialize, Debug)]
pub struct App <'a>{
    /// config:
    /// - blockchain_file,
    /// - node_settings,
    pub config: Config<'a>,
    blockchain: BlockChain<Message>,
    node: Node<String>,
//    miner: Miner<'a>,
}

#[allow(dead_code)]
impl<'a> App<'a> {
    pub fn create(config:Config<'a>) -> Result<Self, Box<dyn Error>> {
        let uri = config.get_value(KEY_BLOCKCHAIN_FILE)
            .ok_or("Blockchain backup file is not given")?;

        let blockchain = match App::load_blockchain(uri){
            Ok(chain) => {
                println!("Loaded blockchain from: {}", uri);
                chain
            },
            Err(err) => {
                println!("Couldn't load from file. Creating new blockchain. \n\t[{}]", err);
                BlockChain::new()
            },
        };

        let this = Self {
            config,
            blockchain,
            node: Node::new(),
        };

        Ok(this)
    }

    fn verify_config(config: &Config<'a>) -> Result<(), Box<dyn Error>>{
        let app_configs = vec![
            KEY_APP_USER,         // will be used as `sender` in message
            KEY_MINER_URL,
            KEY_BLOCKCHAIN_FILE,
        ];

        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn connect_to_network(&self) -> Result<(), Box<dyn Error>> {
        self.node.connect()
    }


    /////////////////////////////// App /////////////////////////////////



    pub fn get_app_info(&self) -> AppInfo{
        let username = self.config.get_value(KEY_APP_USER)
            .unwrap();

        let blockchain_uuid = self.blockchain.get_uuid();

        let user_messages = self.get_messages(Some(username))
            .unwrap_or(vec![]);

        let block_rewards = user_messages.iter()
            .filter(|msg| {
                let sender = Uuid::parse_str(msg.sender.as_str());
                sender.is_ok() && &sender.unwrap() == blockchain_uuid
            })
            .map(|_| BLOCK_REWARD)
            .sum::<usize>();

        let message_rewards = user_messages.iter()
            .filter(|msg| &msg.receiver == username)
            .map(|_| MESSAGE_RECEIVE_REWARD)
            .sum::<usize>();

        let messages_costs = user_messages.iter()
            .filter(|msg| &msg.sender == username)
            .map(|_| MESSAGE_SEND_COST)
            .sum::<usize>();

        let user_balance = (block_rewards + message_rewards) as i64 - messages_costs as i64;

        let chain_len = self.get_last_block().unwrap().get_index();

        AppInfo{
            username,
            user_balance,
            blockchain_uuid,
            chain_len,
            block_reward: BLOCK_REWARD,
            message_read_reward: MESSAGE_RECEIVE_REWARD,
            message_send_cost: MESSAGE_SEND_COST,
        }
    }


    /////////////////////////////// Messages /////////////////////////////////

    /// Get pending messages.
    pub fn get_pending_messages(&self) -> Option<&[Message]> {
        let pending = self.blockchain.get_pending_transactions();

        match pending.is_empty() {
            false => Some(pending),
            true => None
        }
    }

    /// Add given message to the pending message list.
    pub fn add_message(&mut self, message: MessageTemplate) -> Result<(), Box<dyn Error>> {
        let app_info = self.get_app_info();
        let user_balance = app_info.user_balance;

        if user_balance < MESSAGE_SEND_COST as i64 {
            return Err(Box::new(AppError::new("Insufficient funds.")));
        }

        let message= Message {
            id: TransactionID::new(self.blockchain.get_uuid().clone()),
            sender: self.config.get_value(KEY_APP_USER).unwrap().clone(),
            receiver: message.receiver,
            value: message.value,
            load: message.text,
        };
        self.blockchain.add_transaction(message);

        Ok(())
    }

    /// Get given user messages.
    pub fn get_messages(&self, user: Option<&str>) -> Option<Vec<&Message>> {
        let user = user.unwrap_or(
            self.config.get_value(KEY_APP_USER)
            .unwrap()
        );
        // Collection where users messages will be stored
        let mut messages= Vec::new();

        // Iterate through each block
        for block in &self.blockchain.chain {
            // collect all user messages from the block.
            let mut block_messages = block.load.iter()
                .filter(|message| &message.sender == user || &message.receiver == user)
                .collect::<Vec<_>>();
            // move found messages
            messages.append(&mut block_messages);
        }

        match messages.is_empty() {
            true => None,
            false => Some(messages)
        }
    }



    /////////////////////////////// Block /////////////////////////////////

    /// Adds block to blockchain.
    pub fn add_block(&mut self, block: Block<Message>) -> Result<(), Box<dyn Error>> {
        self.blockchain.add_block(block).map(|_| ())
    }

    /// Generate new next block (for mining)
    /// This `Block` needs to be mined before it can be added to `BlockChain`.
    pub fn generate_next_block(&mut self) -> Block<Message> {
        self.blockchain.generate_next_block()
    }

    /// Get genesis block.
    pub fn get_genesis_block(&self) -> Result<&Block<Message>, Box<dyn Error>> {
        Ok(self.blockchain.get_block_genesis())
    }

    /// Get last block in the blockchain.
    pub fn get_last_block(&self) -> Result<&Block<Message>, Box<dyn Error>> {
        Ok(self.blockchain.get_block_last())
    }

    /// Gets all blocks starting from the given one.
    pub fn get_blocks_from(&self, index: usize) -> Result<&[Block<Message>], Box<dyn Error>> {
        self.blockchain.get_blocks_starting_at(index)
    }



    /////////////////////////////// Block /////////////////////////////////

    /// Saves `BlockChain` to the file `KEY_BLOCKCHAIN_FILE`
    pub fn save_blockchain(&self) -> Result<(), Box<dyn Error>> {
        let path = self.config.get_value(KEY_BLOCKCHAIN_FILE).ok_or("Couldn't get path")?;
        fs::File::create(path)?;

        let mut file = fs::OpenOptions::new()
            .append(false)
            .write(true)
            .open(path)?;

        let serialized_chain = serde_json::to_string(&self.blockchain)?;
        write!(file, "{}", serialized_chain)?;

        Ok(())
    }

    /// Load `BlockChain` form file `KEY_BLOCKCHAIN_FILE`
    pub fn load_blockchain(uri: &str) -> Result<BlockChain<Message>, Box<dyn Error>> {
        let file = fs::File::open(uri)?;
        let buffered = io::BufReader::new(file);
        let deserialized: BlockChain<Message> = serde_json::from_reader(buffered)?;

        Ok(deserialized)
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
