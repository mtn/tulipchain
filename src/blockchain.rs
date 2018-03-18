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


type Nonce = u32;

#[derive(Serialize)]
struct Block {
    ind: usize,
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction>,
    nonce: Nonce,
    previous_hash: Option<Digest>,
}

pub struct Blockchain {
    pending_transactions: Vec<Transaction>,
    chain: Vec<Block>,

    // Peers gossip to maintain synchronization
    peers: HashSet<u32>,
    id: u32,
}

impl Block {
    // Returns an owned digest of the block
    fn hash(&self) -> Digest {
        let serialized: Vec<u8> = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        digest.to_vec()
    }

    // Finds a nonce to satisfy the mining problem.
    // This function should be called on the last block in the chain before the one
    // we want to add.
    pub fn find_nonce(&self, previous_nonce: Nonce) -> Nonce {
        let block_hash = &self.hash();

        let mut nonce = 0;
        while !Blockchain::is_valid_nonce(previous_nonce, nonce, block_hash) {
            nonce += 1;
        }

        nonce
    }
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

    pub fn is_valid_nonce(last: Nonce, current: Nonce, prev_digest: &Digest) -> bool {
        // Compute the digest
        let serialized = serialize(&(last, current, prev_digest)).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        &digest[0..4] == &[0,0,0,0]
    }
}
