use std::collections::HashSet;
use super::transaction::{Transaction};

struct Block {

}

struct Blockchain {
    pending_transactions: Vec<Transaction>,
    chain: Vec<Block>,

    // Peers gossip to maintain synchronization
    peers: HashSet<String>,
    id: String,
}

impl Blockchain {
    fn new() -> Blockchain {
        unimplemented!();
    }

    fn register_peer(&mut self, id: String) {
        self.peers.insert(id);
    }
}
