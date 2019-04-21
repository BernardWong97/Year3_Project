//!
//! App API for controlling `App` using HTTP requests.
//!
//! # author: Mindaugas Sharskus
//! # date: 20-04-2019
//!
//! ToDo:
//! - remove `'static` lifetimes.


#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::{Request, Rocket, response::content, data::Data, State};
use rocket_contrib::json::{JsonValue, Json};
use std::io::Read;
use std::sync::Mutex;
use std::fs;

use app::{App, AppError};
use app::config::Config;
use block_chain::transaction::Transaction;

/// App settings file
const CONFIG_FILE:&'static str = "settings_file.txt";



#[post("/transactions", format = "json", data = "<transaction>")]
fn add_transaction(transaction: Json<Transaction<String>>, app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("App Lock");
    let tr = transaction.0;
    app.add_transaction(tr);

    json!({ "status":"ok" })
}

#[get("/transactions")]
fn get_transactions(app: State<Mutex<App>>) -> JsonValue {
    let mut app = app.lock().expect("App Lock");
    let transactions = app.get_pending_transactions().unwrap();

    json!(transactions)
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
fn rocket() -> Result<Rocket, AppError> {
    let config = match Config::from(CONFIG_FILE){
        Ok(content) => content,
        // ToDo: fix this ugliness
        Err(err) => {
            fs::File::create(CONFIG_FILE);
            Config::from(CONFIG_FILE)?
        }
    };
    let app = App::create(config)?;
    let rocket = rocket::ignite()
        .mount("/app", routes![world, add_transaction, get_transactions])
        .manage(Mutex::new(app));

    Ok(rocket)
}

///
/// Launches "Rocket"
///
fn main() -> Result<(), AppError> {
    let rocket:Rocket = rocket()?;
    rocket.launch();

    Ok(())
}