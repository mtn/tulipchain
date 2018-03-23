use sodiumoxide::crypto::sign;
use super::{
    Tulips,
    PublicKey,
    PrivateKey,
};

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

            balance: 0
        }
    }

    // TODO check account balance and stuff
    // Returns a signed transaction
    pub fn new_transaction(&self, value: Tulips, recipient_addr: PublicKey)
        -> Transaction {

            let mut transaction = Transaction {
                sender_addr: Some(self.public_key),
                recipient_addr,
                value,
                signed_digest: None,
            };

            transaction.sign(&self.private_key);

            transaction
        }
}
