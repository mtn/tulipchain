#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate sodiumoxide;
extern crate serde_json;
extern crate argparse;
extern crate bincode;
extern crate reqwest;
extern crate rocket;
extern crate chrono;
extern crate rand;

mod transaction;
mod blockchain;
mod address;

#[cfg(test)]
mod test;

use argparse::{ArgumentParser, Store};
use sodiumoxide::crypto::sign;
use std::collections::HashSet;
use transaction::Transaction;
use blockchain::Blockchain;
use rocket::fairing::AdHoc;
use rocket_contrib::Json;
use std::process::exit;
use std::sync::RwLock;
use rocket::State;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

type PublicKey = sign::ed25519::PublicKey;
type PrivateKey = sign::ed25519::SecretKey;

type SignedDigest = Vec<u8>;
type Digest = Vec<u8>;
type Tulips = u32;

struct ServerConfig {
    address: String,
    port: u16,
}


// Endpoint that returns the full serialized chain of that node's blockchain
#[get("/blockchain/full")]
fn full_blockchain(blockchain: State<RwLock<Blockchain>>) -> Json<Blockchain> {
    Json(blockchain.read().unwrap().clone())
}

#[get("/join")]
fn join(blockchain: State<RwLock<Blockchain>>, addr: SocketAddr) -> Json<Blockchain> {
    // Clone the blockchain before adding the new node to the peer list
    let mut to_transmit = blockchain.read().unwrap().clone();

    // Empty the peer list before transmitting the blockchain.
    to_transmit.peers = HashSet::new();

    // Add the source address to the list of peers
    let mut block_writer = blockchain.write().unwrap();

    let addr_ip_str = addr.ip().to_string();
    let source_ip = if addr_ip_str == "::1" {
        "localhost".to_string()
    } else {
        addr_ip_str
    };

    let source_address = format!("{}:{}", source_ip, addr.port());
    block_writer.register_peer(source_address);

    Json(to_transmit)
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
    let mut base_addr = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Tulipchain runner");
        ap.refer(&mut base_addr)
            .add_option(&["--connect"], Store,
                        "node address to connect to (base url)");
        ap.parse_args_or_exit();
    }

    let chain: RwLock<Blockchain> = if base_addr.is_empty() {
        println!("No input node provided, creating new blockchain instance");
        RwLock::new(Blockchain::new())
    } else {
        // Try to get blockchain from the source node using a http request
        let join_url = format!("{}/join", base_addr);

        let request_result = reqwest::get(&join_url);
        if let Err(_) = request_result {
            println!("An error occured while making a request to the source node");
            exit(1);
        }

        // Look into the text of the request result
        let request_text_result = request_result.unwrap().text();
        if let Err(_) = request_text_result {
            println!("An error occured while reading the response from the source node");
            exit(1);
        }

        // Text holds the serialized blockchain
        let text = request_text_result.unwrap();

        let deserialized: Result<blockchain::Blockchain, _> = serde_json::from_str(&text);
        if let Err(_) = deserialized {
            println!("An error occured while deserializing the blockchain");
            exit(1);
        }

        let mut chain = deserialized.unwrap();
        chain.register_peer(base_addr);

        RwLock::new(chain)
    };

    rocket::ignite()
        .attach(AdHoc::on_attach(|rocket| {
            let config = rocket.config().clone();

            return Ok(rocket.manage(ServerConfig {
                address: config.address,
                port: config.port,
            }))
        }))
        .manage(chain)
        .mount("/", routes![index,
               join,
               full_blockchain,
               new_transaction,
               mine_block])
        .launch();
}
