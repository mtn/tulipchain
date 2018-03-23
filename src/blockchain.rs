use super::transaction::Transaction;
use super::{
    Digest,
    PublicKey,
    ServerConfig,
    Address,
};

use reqwest::header::{Headers, ContentType};
use sodiumoxide::crypto::hash;
use std::collections::HashSet;
use bincode::serialize;
use chrono::prelude::*;
use std::process::exit;
use std::sync::RwLock;
use serde_json;
use reqwest;


type Nonce = u32;
type NodeAddr = String;

const GENESIS_PREV_NONCE: u32 = 0;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    ind: usize,
    timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: Option<Digest>,

    // The coinbase hash can be verified
    pub coinbase_hash: Digest,

    #[serde(skip)]
    pub nonce: Nonce,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blockchain {
    pub pending_transactions: Vec<Transaction>,
    pub chain: Vec<Block>,

    // Peers gossip to maintain synchronization
    pub peers: HashSet<NodeAddr>,

    // Each node holds an address in order to be paid, etc.
    #[serde(skip)]
    pub address: Address,
}

impl Block {
    // Returns an owned digest of the block
    fn hash(&self) -> Digest {
        let serialized: Vec<u8> = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        digest.to_vec()
    }

    // Check that all the transactions inside a block are valid
    pub fn validate_transactions(&self) -> bool {
        for transaction in &self.transactions {
            if !transaction.verify_digest() {
                return false
            }
        }

        true
    }
}

impl Blockchain {
    // Creates a new blockchain, including a genesis block.
    // The genesis block (including nonce) will be identical for all new chains.
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            pending_transactions: vec![],
            chain: vec![],

            peers: HashSet::new(),
        };

        // Create the genesis block and start the chain
        let genesis_block = Block::create_genesis_block();
        blockchain.chain.push(genesis_block);

        blockchain
    }

    pub fn create_genesis_block(&self) -> Block {
        let coinbase_transaction = Transaction {
            sender_addr: None,
            recipient_addr: PublicKey,
            value: Tulips,
            signed_digest: Option<SignedDigest>,
        };

        let mut genesis_block = Block {
            ind: 0,
            timestamp: Utc::now(),
            transactions: vec![],
            previous_hash: None,
            nonce: 0,
            coinbase_hash,
        };

        // The previous nonce of the genesis block is set by convention
        genesis_block.nonce = Blockchain::find_nonce(GENESIS_PREV_NONCE,
                                                     &genesis_block.previous_hash);

        genesis_block
    }


    pub fn init_chain(base_addr: String, config: &ServerConfig) -> RwLock<Blockchain> {
        if base_addr.is_empty() {
            println!("No input node provided, creating new blockchain instance");
            return RwLock::new(Blockchain::new())
        }

        // Try to get blockchain from the source node using a http request
        let join_url = format!("{}/join", base_addr);

        // Create a client and send a request to to join
        let client = reqwest::Client::new();
        let serialized_config = serde_json::to_string(config).unwrap();
        println!("serialized config {}", serialized_config);

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        let request_result = client.post(&join_url)
                                   .headers(headers)
                                   .body(serialized_config)
                                   .send();

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
        println!("text {}", text);

        let deserialized: Result<Blockchain, _> = serde_json::from_str(&text);
        if let Err(_) = deserialized {
            println!("An error occured while deserializing the blockchain");
            exit(1);
        }

        let mut chain = deserialized.unwrap();
        chain.register_peer(base_addr);

        RwLock::new(chain)
    }

    // Registers a new mining peer
    pub fn register_peer(&mut self, addr: NodeAddr) {
        self.peers.insert(addr);
    }

    // Verifies the transaction signature and adds it to the
    // list of pending transactions
    pub fn append_transaction(&mut self, transaction: Transaction) -> bool {
        // Check that the transaction is valid
        if !transaction.verify_digest() {
            return false
        }

        // Since it is, append it to the list of pending transaction
        self.pending_transactions.push(transaction);

        true
    }

    // Appends a new block to the chain, or starts the chain
    pub fn append_block(&mut self, nonce: u32, previous_hash: Option<Digest>) {
        let new_block = Block {
            ind: self.chain.len() + 1,
            timestamp: Utc::now(),
            transactions: self.pending_transactions.clone(),
            nonce,
            previous_hash: {
                if let Some(digest) = previous_hash {
                    // If one was passed as an argument, use it instead of
                    // looking at the chain
                    Some(digest)
                } else {
                    if let Some(block) = self.chain.last() {
                        // Compute the digest of the last block
                        let digest = block.hash();

                        Some(digest)
                    } else {
                        // There should always be a previous hash.
                        // For the genesis block, one is chosen arbitrarily and passed
                        // as an argument. Otherwise, there is always an earlier block.
                        panic!("No previous hash!")
                    }
                }
            },
        };

        // Empty the list of pending transactions
        self.pending_transactions = vec![];

        // Push the confirmed transactions onto the chain
        self.chain.push(new_block);
    }

    // Checks if a nonce is valid according to the mining condition
    pub fn is_valid_nonce(last: Nonce, current: Nonce, prev_digest: &Option<Digest>)
        -> bool {

        // Compute the digest
        let serialized = serialize(&(last, current, prev_digest)).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        // Requiring more than the first 2 digits to be zeros resulting in very large
        // time to find the nonce for a toy implementation.
        &digest[0..2] == &[0,0]
    }

    // Checks whether the chain is valid or not by check the nonce of each block
    pub fn is_valid_chain(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            // The genesis block is handled explicitly because of it's hardcoded
            // previous nonce.
            if i == 0 {
                if !Self::is_valid_nonce(GENESIS_PREV_NONCE,
                                         block.nonce,
                                         &block.previous_hash) {
                    return false
                }
            } else {
                let previous_nonce = self.chain[i-1].nonce;
                if !Self::is_valid_nonce(previous_nonce,
                                         block.nonce,
                                         &block.previous_hash) {
                    return false
                }
            }
        }

        true
    }

    // Private function to create a coinbase transaction when blocks are mined
    fn create_coinbase_transaction(recipient_addr: PublicKey) -> Transaction {
        Transaction {
            sender_addr: None,
            recipient_addr,
            value: 1,
            signed_digest: None,
        }
    }

    // Finds a nonce that satisfies the mining condition for the next block.
    // Note: it doesn't depend on the contents of the block that being added.
    pub fn find_nonce(previous_nonce: Nonce, previous_hash: &Option<Digest>)
        -> Nonce {
        // If there is no previous hash, then the block isn't getting added onto a chain
        let mut nonce = 0;
        while !Blockchain::is_valid_nonce(previous_nonce, nonce, previous_hash) {
            if nonce % 1000 == 0 {
                println!("Searching for nonce {}", nonce);
            }
            nonce += 1;
        }

        println!("Nonce found: {}", nonce);
        nonce
    }

    pub fn get_last_nonce(&self) -> Nonce {
        // There will always be at least one block, so it's safe to unwrap
        let last_block = self.chain.last().unwrap();

        last_block.nonce
    }

    pub fn transmit_message(&self) {
        unimplemented!();
    }
}
