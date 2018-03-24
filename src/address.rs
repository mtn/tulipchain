use sodiumoxide::crypto::sign;
use super::{PrivateKey, PublicKey, Tulips};

use transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Address {
    pub public_key: PublicKey,
    private_key: PrivateKey,

    pub balance: Tulips,
}

impl Address {
    pub fn new() -> Address {
        let (public_key, private_key) = sign::gen_keypair();

        Address {
            public_key,
            private_key,

            balance: 0,
        }
    }

    // Returns a signed transaction, assuming the sender has enough tulips
    pub fn new_transaction(
        &mut self,
        value: Tulips,
        recipient_addr: PublicKey,
    ) -> Option<Transaction> {
        // Transactions should still be forged easily, so basic checks are
        // verified again before being added to the blockchain
        if value < 0 {
            return None;
        }

        if self.balance < value {
            return None;
        }

        let mut transaction = Transaction::new(Some(self.public_key), recipient_addr, value);

        // Sign the transaction and update the balance
        transaction.sign(&self.private_key);
        self.balance -= value;

        Some(transaction)
    }
}
