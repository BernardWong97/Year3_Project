//! Made by following tutorial:
//! https://www.youtube.com/watch?v=U8GGZ4TqlQs&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW&index=18
//!
//! Ths block-chain:
//!     - prof-of-work
//!     - not distributed (single bloc-chain)
//!

#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty= String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("We need integer");
    println!("generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("[1] - New Transaction");
        println!("[2] - Mine block");
        println!("[3] - Change Difficulty");
        println!("[4] - Change Reward");
        println!("[0] - Exit");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            // Exit
            0 => {
                println!("exiting");
                process::exit(0);
            },
            // New Transaction
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter senders address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("Enter receivers address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap()
                );

                match res {
                    true => println!("... transaction added"),
                    false => println!("... transaction failed"),
                }
            },
            // Mine block
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("... block generated successfully"),
                    false => println!("... block generation failed"),
                }
            },
            // Change Difficulty
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("... updated difficulty successfully"),
                    false => println!("... failed to updated difficulty"),
                }
            },
            // Change Reward
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_difficulty(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("... updated reward successfully"),
                    false => println!("... failed to updated reward"),
                }
            },
            _ => println!("Invalid option please retry!"),
        }

    }
}
