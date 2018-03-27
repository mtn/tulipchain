#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate argparse;
extern crate bincode;
extern crate chrono;
extern crate rand;
extern crate reqwest;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sodiumoxide;

mod transaction;
mod blockchain;
mod address;

#[cfg(test)]
mod test;

use transaction::{PartialTransaction, Transaction};
use argparse::{ArgumentParser, Store};
use sodiumoxide::crypto::sign;
use std::collections::HashSet;
use blockchain::Blockchain;
use rocket::fairing::AdHoc;
use rocket_contrib::Json;
use std::sync::RwLock;
use rocket::State;

type PublicKey = sign::ed25519::PublicKey;
type PrivateKey = sign::ed25519::SecretKey;

type SignedDigest = Vec<u8>;
type Digest = Vec<u8>;
type Tulips = u32;

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    address: String,
    port: u16,
}

// Endpoint that returns the full serialized chain of that node's blockchain
#[get("/blockchain/full")]
fn full_blockchain(blockchain: State<RwLock<Blockchain>>) -> Json<Blockchain> {
    Json(blockchain.read().unwrap().clone())
}

#[post("/network/join", data = "<addr>")]
fn join(blockchain: State<RwLock<Blockchain>>, addr: Json<ServerConfig>) -> Json<Blockchain> {
    // Clone the blockchain before adding the new node to the peer list
    let mut to_transmit = blockchain.read().unwrap().clone();

    // Empty the peer list before transmitting the blockchain.
    to_transmit.peers = HashSet::new();

    // Add the source address to the list of peers
    let mut block_writer = blockchain.write().unwrap();

    let source_config = addr.into_inner();
    let mut source_str = format!("{}:{}", source_config.address, source_config.port);
    if !source_str.contains("http://") {
        source_str = format!("http://{}", source_str);
    }

    block_writer.register_peer(source_str);

    Json(to_transmit)
}

// Endpoint to receive new transactions from the network
#[post("/network/transactions/new", data = "<transaction_data>")]
fn new_transaction_from_network(
    blockchain: State<RwLock<Blockchain>>,
    transaction_data: Json<Transaction>,
) {
    let transaction = transaction_data.into_inner();
    let mut block_writer = blockchain.write().unwrap();

    block_writer.append_transaction(transaction);
}

#[post("/transactions/new", data = "<transaction_data>")]
fn new_transaction(
    blockchain: State<RwLock<Blockchain>>,
    transaction_data: Json<PartialTransaction>,
) -> Json<Option<Transaction>> {
    let partial_transaction = transaction_data.into_inner();
    let mut block_writer = blockchain.write().unwrap();
    let mut node_addr = block_writer.address.clone().unwrap();

    let transaction = node_addr.new_transaction(
        partial_transaction.value,
        partial_transaction.recipient_addr,
    );

    if let None = transaction {
        return Json(None);
    }

    if !block_writer.append_transaction(transaction.clone().unwrap()) {
        return Json(None);
    }

    block_writer.broadcast_transaction(transaction.clone().unwrap());

    Json(transaction)
}

#[get("/mine")]
fn mine_block(blockchain: State<RwLock<Blockchain>>) -> String {
    String::from("Mining a new block")
}

// Endpoint to report new nonce
#[get("/blocks/new")]
fn add_block(blockchain: State<RwLock<Blockchain>>) -> String {
    String::from("Mining a new block")
}

#[get("/")]
fn index(blockchain: State<RwLock<Blockchain>>) -> String {
    unimplemented!()
}

fn main() {
    let mut base_addr = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Tulipchain runner");
        ap.refer(&mut base_addr).add_option(
            &["--connect"],
            Store,
            "node address to connect to (base url)",
        );
        ap.parse_args_or_exit();
    }

    rocket::ignite()
        .attach(AdHoc::on_attach(move |rocket| {
            let config = rocket.config().clone();
            let server_config = ServerConfig {
                address: config.address,
                port: config.port,
            };

            let chain: RwLock<Blockchain> =
                Blockchain::init_chain(base_addr.clone(), &server_config);
            return Ok(rocket.manage(chain));
        }))
        .manage(reqwest::Client::new())
        .mount(
            "/",
            routes![
                index,
                join,
                full_blockchain,
                new_transaction,
                new_transaction_from_network,
                mine_block,
                add_block
            ],
        )
        .launch();
}
