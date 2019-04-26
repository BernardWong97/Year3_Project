//!
//! App API for controlling `App` using HTTP requests.
//!
//! # author: Mindaugas Sharskus
//! # date: 20-04-2019
//!
//! ToDo (improvements):
//! - ?? get transactions from particular time stamp instead of getting all pending.
//! - Create custom error for pass error information.
//! - Error should implement rocket::Response
//! - add endpoint to create message from given details


#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate rocket_cors;

use reqwest::Client;

use rocket::{Request, Rocket, response::content, data::Data, State, Response};
use rocket_contrib::json::{JsonValue, Json};
use rocket::config::{Environment};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

use std::error;
use std::io::Read;
use std::sync::Mutex;
use std::fs;
use std::env::join_paths;
use std::ptr::read;

use app::{App, AppError, MessageTemplate};
use app::config::Config;
use app::Message;
use block_chain::hashable::Hashable;
use block_chain::transaction::Transaction;
use block_chain::Block;
use block_chain::block_header::BlockHeader;
use miner::Miner;




////////////////////// Messages /////////////////////////////////

#[post("/message", format = "json", data = "<message>")]
/// Adds Message to the pending list
fn add_message(message: Json<MessageTemplate>, app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("Message: App Lock.");

    match app.add_message(message.0) {
        Ok(_) => json!({ "status":"ok", "msg":"Message was added to the message queue." }),
        Err(_err) => json!({ "status":"err", "err": "Couldn't add message to the pending list" })
    }
}

#[get("/message")]
/// Gets all pending messages
fn get_pending_messages(app: State<Mutex<App>>) -> JsonValue {
    let app = app.lock().expect("Message: App Lock.");

    match app.get_pending_messages() {
        Some(msg) => json!(msg),
        None => json!({"status":"err", "err":"No pending messages"})
    }
}

#[get("/message/<user>")]
/// Gets all given user messages (sent & received)
fn get_messages_user(user: String, app: State<Mutex<App>>) -> Option<JsonValue> {
    let app = app.lock().expect("Message: App Lock");

    app.get_messages(user.as_str()).map(|val| json!(val))
}




////////////////////// Block ////////////////////////////////

#[post("/block", format = "json", data = "<block>")]
/// Adds given block to the blockchain.
fn add_blocks(block: Json<Block<Message>>, app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("Block: App Lock");

    match app.add_block(block.0) {
        Ok(()) => json!({"status":"ok"}),
        Err(_err) =>             // ToDo: add reason why add block failed
            json!({
                "status":"err",
                "err":"Couldn't add block to blockchain: [TODO: add reason]"
            })
    }
}

#[get("/block/<index>")]
/// Gets all blocks from the given one
fn get_blocks_starting(index: usize, app: State<Mutex<App>>) -> JsonValue {
    let app = app.lock().expect("Block: App Lock");

    match app.get_blocks_from(index) {
        Ok(blocks) => json!(blocks),
        Err(_err) =>         // ToDo: add reason why retrieving blocks failed
            json!({
                "status":"err",
                "err":"Couldn't retrieve blocks: [todo: add reason]"
            }),
    }
}

#[get("/block/genesis")]
/// Gets genesis block.
fn get_genesis_block(app: State<Mutex<App>>) -> JsonValue {
    let app = app.lock().expect("Block: App Lock");

    match app.get_genesis_block() {
        Ok(block) => json!({
            "genesis":block,
            "hash":block.hash()
        }),
        Err(_err) =>         // ToDo: add reason why retrieving blocks failed
            json!({
                "status":"err",
                "err":"Couldn't retrieve genesis blocks: [todo: add reason]"
            }),
    }
}

#[get("/block/last")]
/// Gets last block in blockchain.
fn get_last_block(app: State<Mutex<App>>) -> JsonValue {
    let app = app.lock().expect("Block: App Lock");

    match app.get_last_block() {
        Ok(block) => json!({
            "genesis":block,
            "hash":block.hash()
        }),
        Err(_err) =>         // ToDo: add reason why retrieving blocks failed
            json!({
                "status":"err",
                "err":"Couldn't retrieve last blocks: [todo: add reason]"
            }),
    }
}

#[get("/block/new")]
/// Creates new block. Block includes all pending messages.
/// Block should be send to miner.
fn generate_new_block(app: State<Mutex<App>>) -> JsonValue {
    // mut because moves all pending transactions
    let mut app = app.lock().expect("Miner: App Lock");
    let block = app.generate_next_block();

    json!(block)
}


////////////////////// Miner ////////////////////////////////

#[post("/miner", format = "application/json", data = "<block_header>")]
/// Mines given block (finds nonce).
/// Miner is on different location (separated from app)
fn get_nonce(block_header: Json<BlockHeader>, app: State<Mutex<App>>) -> Option<JsonValue> {
    let app = app.lock().expect("Miner: App Lock");
    let header:BlockHeader = block_header.0;

    let miners_url = app.config.get_value(app::KEY_MINER_URL)?.as_str();

    Client::new()
        .post(miners_url)
        .json(&header)
        .send().unwrap()    // todo: fix unwrap()
        .json().unwrap()    // todo: fix unwrap()
}



////////////////////// Blockchain ////////////////////////////////

#[get("/blockchain/<command>")]
/// Control blockchain:
/// - `"backup"` - saves blockchain to file
/// - `"greeting"` - says "hi" (for testing)
///
/// ToDo:
/// - ?? Control App ??
fn blockchain_control(command: String, app: State<Mutex<App>>) -> Option<JsonValue> {
    let app = app.lock().expect("Blockchain: App Lock");

    match command.as_str() {
        "backup" => { app.save_blockchain(); Some(json!({"status":"ok"})) },
        "greeting" => Some(json!({"status":"ok", "greeting":"Hi, from blockchain"})),
        _ => Some(json!({"status":"err", "err":"unsupported command"}))
    }
}



////////////////////// App ////////////////////////////////

#[get("/info")]
/// Gets app info
fn get_app_info(app: State<Mutex<App>>) -> JsonValue {
    let app = app.lock().expect("Block: App Lock");


    json!(app.get_app_info())
}

#[get("/auto")]
/// Automatically generates block (gets new -> mines -> adds to blockchain)
fn auto_block(app: State<Mutex<App>>) -> Option<JsonValue> {
    let mut app = app.lock().expect("Blockchain: App Lock");
    let mut block = app.generate_next_block();

    let nonce = Miner::new(&block.header).start_thread().join().unwrap();

    block.header.set_nonce(nonce);
    app.add_block(block).ok()?;

    Some(json!({"status":"ok"}))
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


    let (allowed_origins, failed_origins) = AllowedOrigins::some(&[
        "https://www.acme.com",
        "http://localhost:8080",
    ]);

    // You can also deserialize this
    let cors = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allow_credentials: true,
        ..Default::default()
    };



    let config = match Config::from(app::CONFIG_FILE){
        Ok(content) => content,
        Err(_err) => {
            fs::File::create(app::CONFIG_FILE)?;
            Config::from(app::CONFIG_FILE)?
        }
    };
    let app = App::create(config)?;
    let config = rocket::config::Config::build(Environment::Staging)
        .port(8001)     // change Rocket port
        .finalize()?;

    let rocket = rocket::custom(config)
        .mount("/app", routes![
            world,
            // message
            add_message, get_pending_messages, get_messages_user,
//            add_message_o,
            // blocks
            add_blocks, get_blocks_starting,
            get_genesis_block, get_last_block, generate_new_block,
            // miner
            get_nonce,
            blockchain_control,
            // app
            get_app_info, auto_block,
    ])
        .manage(Mutex::new(app))
        .attach(cors);

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