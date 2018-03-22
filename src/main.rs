#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate sodiumoxide;
extern crate serde_json;
extern crate argparse;
extern crate bincode;
extern crate rocket;
extern crate chrono;
extern crate rand;

mod transaction;
mod blockchain;
mod address;

#[cfg(test)]
mod test;

use argparse::{ArgumentParser, Store};
use rocket_contrib::{Json, Value};
use sodiumoxide::crypto::sign;
use transaction::Transaction;
use blockchain::Blockchain;
use rocket::State;

type PublicKey = sign::ed25519::PublicKey;
type PrivateKey = sign::ed25519::SecretKey;

type SignedDigest = Vec<u8>;
type Digest = Vec<u8>;
type Tulips = u32;


// Endpoint that returns the full serialized chain of that node's blockchain
#[get("/blockchain/full")]
fn full_blockchain(blockchain: State<Blockchain>) -> Json<Blockchain> {
    Json(blockchain.clone())
}

#[get("/transactions/new")]
fn new_transaction(blockchain: State<Blockchain>) -> Json<Transaction> {
    unimplemented!()
    // serde_json::to_string(&blockchain.chain).unwrap()
}

#[get("/mine")]
fn mine_block(blockchain: State<Blockchain>) -> String {
    serde_json::to_string(&blockchain.chain).unwrap();
    String::from("Mining a new block")
}

#[get("/")]
fn index(blockchain: State<Blockchain>) -> String {
    unimplemented!()
}

fn main() {

    rocket::ignite().manage(Blockchain::new())
                    .mount("/", routes![index,
                                        full_blockchain,
                                        new_transaction,
                                        mine_block]).launch();
}
