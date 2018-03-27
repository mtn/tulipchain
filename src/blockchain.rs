use super::transaction::Transaction;
use super::address::Address;
use super::{Digest, PublicKey, ServerConfig, Tulips};

use reqwest::header::{ContentType, Headers};
use sodiumoxide::crypto::hash;
use std::collections::{HashMap, HashSet};
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
    pub coinbase_transaction: Transaction,

    #[serde(skip)]
    pub nonce: Nonce,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Blockchain {
    pub pending_transactions: Vec<Transaction>,
    pub chain: Vec<Block>,

    // Each node holds an address in order to receive transaction, etc.
    // Note: every blockchain will have an address, but this is wrapped
    // in an option type solely because PrivateKey has no default value
    // for deserialization
    #[serde(skip)]
    pub address: Option<Address>,

    // Each node maintains a dictionary holding balances of all other nodes
    // In a persistent implementation, this might involve a database
    #[serde(skip)]
    pub address_balances: HashMap<PublicKey, Tulips>,

    // Peers gossip to maintain synchronization
    pub peers: HashSet<NodeAddr>,
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
                return false;
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

            address: Some(Address::new()),
            address_balances: HashMap::new(),
            peers: HashSet::new(),
        };

        // Create the genesis block and start the chain
        let genesis_block = blockchain.create_genesis_block();
        blockchain.chain.push(genesis_block);

        blockchain
    }

    pub fn create_genesis_block(&self) -> Block {
        let coinbase_transaction =
            Transaction::create_coinbase_transaction(self.address.clone().unwrap().public_key);

        let mut genesis_block = Block {
            ind: 0,
            timestamp: Utc::now(),
            transactions: vec![],
            previous_hash: None,
            nonce: 0,
            coinbase_transaction,
        };

        // The previous nonce of the genesis block is set by convention
        genesis_block.nonce =
            Blockchain::find_nonce(GENESIS_PREV_NONCE, &genesis_block.previous_hash);

        genesis_block
    }

    pub fn init_chain(base_addr: String, config: &ServerConfig) -> RwLock<Blockchain> {
        if base_addr.is_empty() {
            println!("No input node provided, creating new blockchain instance");
            return RwLock::new(Blockchain::new());
        }

        // Try to get blockchain from the source node using a http request
        let join_url = format!("{}/join", base_addr);

        // Create a client and send a request to to join
        let client = reqwest::Client::new();
        let serialized_config = serde_json::to_string(config).unwrap();

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        let request_result = client
            .post(&join_url)
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
        println!("Registering peer with address {}", addr);

        self.peers.insert(addr);
    }

    // Verifies the transaction signature and adds it to the
    // list of pending transactions
    pub fn append_transaction(&mut self, transaction: Transaction) -> bool {
        // Check that the transaction is valid
        if !transaction.verify_digest() {
            return false;
        }

        if let None = transaction.sender_addr {
            // The transaction is a coinbase, so we look through blocks to find it
            let mut found = false;
            for block in self.chain.iter().rev() {
                if transaction == block.coinbase_transaction {
                    found = true;
                }
            }

            // If it wasn't the coinbase for any block already in the chain,
            // then it must be counterfeit (coinbase transactions aren't added using
            // append_transaction)
            if !found {
                return false;
            }
        } else {
            // Otherwise, just verify that it was signed by the sender
            if !transaction.verify_digest() {
                return false;
            }
        }

        let sender_addr = transaction.sender_addr.unwrap();
        if self.address_balances.contains_key(&sender_addr) {
            let balance = self.address_balances.get(&sender_addr).unwrap();
            // Ensure the address has enough tulips
            if *balance < transaction.value {
                return false;
            }
        } else {
            // 0-balance addresses can't send more than 0 tulips
            if transaction.value > 0 {
                return false;
            }

            self.address_balances.insert(sender_addr, 0);
        }

        // Set the recipient balance, if this is the first transaction it's been
        // involved in
        let recipient_addr = transaction.recipient_addr;
        if !self.address_balances.contains_key(&recipient_addr) {
            self.address_balances.insert(recipient_addr, 0);
        }

        // Update the balances
        self.address_balances
            .get_mut(&sender_addr)
            .map(|bal| *bal - transaction.value);
        self.address_balances
            .get_mut(&recipient_addr)
            .map(|bal| *bal + transaction.value);

        // Since it is, append it to the list of pending transaction
        self.pending_transactions.push(transaction);

        true
    }

    // Appends a new block to the chain, or starts the chain
    pub fn append_block(
        &mut self,
        nonce: u32,
        previous_hash: Option<Digest>,
        reward_addr: PublicKey,
    ) {
        let coinbase_transaction = Transaction::create_coinbase_transaction(reward_addr);

        let new_block = Block {
            ind: self.chain.len() + 1,
            timestamp: Utc::now(),
            transactions: self.pending_transactions.clone(),
            nonce,
            coinbase_transaction,
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
    pub fn is_valid_nonce(last: Nonce, current: Nonce, prev_digest: &Option<Digest>) -> bool {
        // Compute the digest
        let serialized = serialize(&(last, current, prev_digest)).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        // Requiring more than the first 2 digits to be zeros resulting in very large
        // time to find the nonce for a toy implementation.
        &digest[0..2] == &[0, 0]
    }

    // Checks whether the chain is valid or not by check the nonce of each block
    pub fn is_valid_chain(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            // The genesis block is handled explicitly because of it's hardcoded
            // previous nonce.
            if i == 0 {
                if !Self::is_valid_nonce(GENESIS_PREV_NONCE, block.nonce, &block.previous_hash) {
                    return false;
                }
            } else {
                let previous_nonce = self.chain[i - 1].nonce;
                if !Self::is_valid_nonce(previous_nonce, block.nonce, &block.previous_hash) {
                    return false;
                }
            }
        }

        true
    }

    // Finds a nonce that satisfies the mining condition for the next block.
    // Note: it doesn't depend on the contents of the block that being added.
    pub fn find_nonce(previous_nonce: Nonce, previous_hash: &Option<Digest>) -> Nonce {
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

    // Broadcast the transaction to each peer in the peer list
    pub fn broadcast_transaction(&self, transaction: Transaction) {
        let client = reqwest::Client::new();

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        for peer in &self.peers {
            let serialized_transaction = serde_json::to_string(&transaction).unwrap();
            client
                .post(peer)
                .headers(headers.clone())
                .body(serialized_transaction)
                .send();
        }
    }
}
