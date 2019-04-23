//!
//! Miner API for controlling miner using HTTP request.
//!
//! Mindaugas Sharskus
//! 20-04-2019
//!
//! ToDo (feature improvements):
//! - Allow multi hash mining (queue jobs)
//! - Allow for checking for mining status instead of waiting until its done.
//! This will be needed for greater difficulty. Idea is: firs summit a job,
//! then at separate request check if job is done (hash is mined).
//! https://rocket.rs/v0.4/guide/state/


#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::{Request, response::content, data::Data};
use rocket_contrib::json::{JsonValue, Json};
use std::io::Read;

use block_chain::block_header::BlockHeader;
use miner::Miner;


///
/// Mines hash for a given `BlockHeader`.
///
#[post("/start", format = "application/json", data = "<block_header>")]
fn get_hash(block_header: Json<BlockHeader>) -> JsonValue {
    let header:BlockHeader = block_header.0;
    let nonce = Miner::new(&header).start();

    json!({
        "status":"ok",
        "nonce":nonce
    })
}

///
/// Mines dummy hash for the given difficulty (testing).
///
#[get("/start/<difficulty>")]
fn random(difficulty: usize) -> JsonValue{
    let mut header = BlockHeader::first();
    header.set_difficulty(difficulty);
    let nonce = Miner::new(&header).start();

    json!({
        "status":"ok",
        "nonce":nonce
    })
}

///
/// Says hello.
///
#[get("/")]
fn world() -> &'static str {
    "Miner says: Hello!"
}

///
/// builds "Rocket"
///
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/miner",
               routes![world, get_hash, random])
}

///
/// Launches "Rocket"
///
fn main() {
    rocket().launch();
}