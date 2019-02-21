//!
//! Implementation of block-chain from:
//! https://freestartupkits.com/articles/technology/cryptocurrency-news-and-tips/ultimate-rust-blockchain-tutorial/
//!


// this would be the library crate
mod rusty_chain;

use std::process;

//use rusty_chain::blockchain::Blockchain;
//use rusty_chain::error::MiningError;
use crate::rusty_chain::MiningError;
use crate::rusty_chain::BlockChain;

fn main() {
    println!("Welcome to Rusty Chain");

    run().
        unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1)
        })
}

fn run() -> Result<(), MiningError> {
    let mut chain = BlockChain::new()?;
    println!("Send 1 RC to foo");
    chain.add_block("cool block bro!")?;

    println!("Send another RC to foo");
    chain.add_block("Testing another new block!")?;

    println!("Send another RC to foo");
    chain.add_block("Testing!")?;

    println!("Traversing blockchain:\n");
    chain.traverse();

    Ok(())
}

