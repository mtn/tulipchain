use super::transaction::Transaction;
use super::SignedDigest;

use std::collections::HashSet;
use rand;

struct Block {

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
}
