use super::transaction::Transaction;
use super::{
    SignedDigest,
    Digest,
    PublicKey,
};

use sodiumoxide::crypto::{sign, hash};
use std::collections::HashSet;
use bincode::serialize;
use chrono::prelude::*;
use rand;


type Nonce = u32;

const GENESIS_PREV_NONCE: u32 = 0;

#[derive(Debug, Serialize)]
pub struct Block {
    ind: usize,
    timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: Option<Digest>,

    #[serde(skip)]
    pub nonce: Nonce,
}

#[derive(Debug)]
pub struct Blockchain {
    pub pending_transactions: Vec<Transaction>,
    pub chain: Vec<Block>,

    // Peers gossip to maintain synchronization
    peers: HashSet<u32>,
    // Identifier of the node running that blockchain instance
    node_id: u32,
}

impl Block {
    // Returns an owned digest of the block
    fn hash(&self) -> Digest {
        let serialized: Vec<u8> = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        digest.to_vec()
    }

    pub fn create_genesis_block() -> Block {
        let mut genesis_block = Block {
            ind: 0,
            timestamp: Utc::now(),
            transactions: vec![],
            previous_hash: None,
            nonce: 0,
        };

        // The previous nonce of the genesis block is set by convention
        genesis_block.nonce = Blockchain::find_nonce(GENESIS_PREV_NONCE,
                                                     &genesis_block.previous_hash);

        genesis_block
    }

    // Check that all the transactions inside a block are valid
    pub fn validate_transactions(&self) -> bool {
        // for transaction in self.transactions {
        // }
        unimplemented!()
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
            node_id: rand::random::<u32>(),
        };

        // Create the genesis block and start the chain
        let genesis_block = Block::create_genesis_block();
        blockchain.chain.push(genesis_block);

        blockchain
    }

    // Registers a new mining peer
    pub fn register_peer(&mut self, id: u32) {
        self.peers.insert(id);
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
            println!("Searching for nonce {}", nonce);
            nonce += 1;
        }

        nonce
    }

    pub fn get_last_nonce(&self) -> Nonce {
        // There will always be at least one block, so it's safe to unwrap
        let last_block = self.chain.last().unwrap();

        last_block.nonce
    }
}
