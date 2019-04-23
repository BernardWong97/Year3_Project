//!
//! App API for controlling `App` using HTTP requests.
//!
//! # author: Mindaugas Sharskus
//! # date: 20-04-2019
//!
//! ToDo (improvements):
//! - get transactions from particular time stamp instead of getting all pending.
//! - Create custom error for pass error information.


#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
use reqwest::Client;

use rocket::{Request, Rocket, response::content, data::Data, State};
use rocket_contrib::json::{JsonValue, Json};
use rocket::config::{Environment};

use std::error;
use std::io::Read;
use std::sync::Mutex;
use std::fs;
use std::env::join_paths;

use app::{App};
use app::config::Config;
use app::Message;
use block_chain::hashable::Hashable;
use block_chain::transaction::Transaction;
use block_chain::Block;
use block_chain::block_header::BlockHeader;
use miner::Miner;
use std::ptr::read;


/// App settings file
const CONFIG_FILE:&'static str = "settings_file.txt";

////////////////////// Transaction /////////////////////////////////

#[post("/transactions", format = "json", data = "<transaction>")]
fn add_transaction(transaction: Json<Message>, app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("App Lock");
    let tr = transaction.0;
    app.add_transaction(tr);

    json!({ "status":"ok" })
}

#[get("/transactions")]
fn get_transactions(app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("Transaction: App Lock.");
    let transactions = app.get_pending_transactions().unwrap();

    json!(transactions)
}



////////////////////// Block ////////////////////////////////

#[post("/block", format = "json", data = "<block>")]
fn add_blocks(block: Json<Block<Message>>, app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("Block: App Lock");
    let block = block.0;

    match app.add_block(block) {
        Ok(()) => json!({"status":"ok"}),
        // ToDo: add reason why add block failed
        Err(err) => json!({
            "status":"err",
            "err":"Couldn't add block to blockchain: [TODO: add reason]"
        })
    }
}

#[get("/block", format = "json", data = "<block>")]
fn get_blocks_starting(block: Json<Block<Message>>, app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("Block: App Lock");
    let block = block.0;

    match app.get_blocks_from(&block) {
        Ok(blocks) => json!(blocks),
        // ToDo: add reason why retrieving blocks failed
        Err(err) =>
            json!({
                "status":"err",
                "err":"Couldn't retrieve blocks: [todo: add reason]"
            }),
    }
}

#[get("/block/genesis")]
fn get_genesis_block(app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("Block: App Lock");

    match app.get_genesis_block() {
        Ok(block) => json!({
            "genesis":block,
            "hash":block.hash()
        }),
        // ToDo: add reason why retrieving blocks failed
        Err(err) =>
            json!({
                "status":"err",
                "err":"Couldn't retrieve genesis blocks: [todo: add reason]"
            }),
    }
}

#[get("/block/last")]
fn get_last_block(app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("Block: App Lock");

    match app.get_last_block() {
        Ok(block) => json!({
            "genesis":block,
            "hash":block.hash()
        }),
        // ToDo: add reason why retrieving blocks failed
        Err(err) =>
            json!({
                "status":"err",
                "err":"Couldn't retrieve last blocks: [todo: add reason]"
            }),
    }
}



////////////////////// Miner ////////////////////////////////

#[post("/miner", format = "application/json", data = "<block_header>")]
fn get_hash(block_header: Json<BlockHeader>, app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("Block: App Lock");
    let header:BlockHeader = block_header.0;

    // todo: fix unwrap()
    let mut miners_url = app.config.get_value(app::KEY_MINER_URL).unwrap().as_str();

    Client::new()
        .post(miners_url)
        .json(&header)
        .send().unwrap()    // todo: fix unwrap()
        .json().unwrap()    // todo: fix unwrap()
}






///
/// Says hello.
///
#[get("/")]
fn world() -> &'static str {
    "App says: Hello!"
}

///
/// builds "Rocket"
///
fn rocket() -> Result<Rocket, Box<dyn error::Error>> {
    let config = match Config::from(CONFIG_FILE){
        Ok(content) => content,
        // ToDo: fix this ugliness
        Err(err) => {
            fs::File::create(CONFIG_FILE);
            Config::from(CONFIG_FILE)?
        }
    };
    let app = App::create(config)?;
    let config = rocket::config::Config::build(Environment::Staging)
        .port(8001)
        .finalize()?;

    let rocket = rocket::custom(config)
        .mount("/app", routes![
            world,
            add_transaction, get_transactions,
            add_blocks, get_blocks_starting, get_genesis_block, get_last_block,
            get_hash,
    ])
        .manage(Mutex::new(app));

    Ok(rocket)
}

///
/// Launches "Rocket"
///
fn main() -> Result<(), Box<dyn error::Error>> {
    let rocket:Rocket = rocket()?;
    rocket.launch();

    Ok(())
}