#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Status;
use primitives::tx::{TransferTx};
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello world and welcome to the RustChain server."
}


#[post("/transfer_tx", format = "json", data = "<tx_input>")]
fn transfer_tx_post(tx_input: Json<TransferTx>) -> Status {
    // 1. Validate Tx
    
    // 2. Send Tx to mempool
}

pub fn run() {
    rocket::ignite().mount("/", routes![index]).launch();
}