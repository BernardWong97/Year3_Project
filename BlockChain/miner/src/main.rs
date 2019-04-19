
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::{Request, response::content, data::Data};

use miner::Miner;
use block_chain::block_header::BlockHeader;
use std::io::Read;
use rocket_contrib::json::{JsonValue, Json};


#[get("/hello")]
fn world() -> &'static str {
    "Hello, world!"
}

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

#[post("/start", format = "application/json", data = "<block_header>")]
fn get_hash(block_header: Json<BlockHeader>) -> JsonValue {

    let header:BlockHeader = block_header.0;
    let nonce = Miner::new(&header).start();

    json!({
        "status":"ok",
        "nonce":nonce
    })
}

fn main() {
    rocket::ignite()
        .mount("/miner",
               routes![world, get_hash, random])
        .launch();
}