use super::transaction::Transaction;
use super::{
    SignedDigest,
    Digest,
};

use sodiumoxide::crypto::{sign, hash};
use std::collections::HashSet;
use bincode::serialize;
use chrono::prelude::*;
use rand;

#[derive(Serialize)]
struct Block {
    ind: usize,
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction>,
    nonce: u32,
    previous_hash: Option<Digest>,
}

pub struct Blockchain {
    pending_transactions: Vec<Transaction>,
    chain: Vec<Block>,

    // Peers gossip to maintain synchronization
    peers: HashSet<u32>,
    id: u32,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            pending_transactions: vec![],
            chain: vec![],

            peers: HashSet::new(),
            id: rand::random::<u32>(),
        }
    }

    // Registers a new mining peer
    pub fn register_peer(&mut self, id: u32) {
        self.peers.insert(id);
    }

    // Verifies the transaction signature and adds it to the
    // list of pending transactions
    pub fn append_transaction(&mut self, transaction: Transaction,
                              signed_digest: SignedDigest) -> bool {
        // Check that the transaction is valid
        if !transaction.verify_digest(signed_digest) {
            return false;
        }

        // Since it is, append it to the list of pending transaction
        self.pending_transactions.push(transaction);

        true
    }

    pub fn append_block(&mut self, nonce: u32, previous_hash: Option<Digest>)
        -> Block {

        let new_block = Block {
            ind: self.chain.len() + 1,
            timestamp: Utc::now(),
            transactions: self.pending_transactions.clone(),
            nonce,
            previous_hash: {
                if let Some(digest) = previous_hash {
                    Some(digest)
                } else {
                    if let Some(block) = self.chain.last() {
                        // Compute the digest
                        let serialized: Vec<u8> = serialize(block).unwrap();
                        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

                        let owned_digest = digest.to_vec();

                        Some(owned_digest)
                    } else {
                        panic!("No previous hash")
                    }
                }
            },
        };

        new_block
    }
}
