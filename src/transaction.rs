use sodiumoxide::crypto::{hash, sign};
use bincode::serialize;
use super::{PrivateKey, PublicKey, SignedDigest, Tulips};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Transaction {
    // Coinbase transactions have no sender
    pub sender_addr: Option<PublicKey>,
    pub recipient_addr: PublicKey,
    pub value: Tulips,

    #[serde(skip)]
    pub signed_digest: Option<SignedDigest>,
}

impl Transaction {
    pub fn new(
        sender_addr: Option<PublicKey>,
        recipient_addr: PublicKey,
        value: Tulips,
    ) -> Transaction {
        Transaction {
            sender_addr,
            recipient_addr,
            value,
            signed_digest: None,
        }
    }

    pub fn sign(&mut self, signing_key: &PrivateKey) {
        let serialized = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        // Sign the digest with the senders private key
        let signed_digest = sign::sign(digest, signing_key);
        self.signed_digest = Some(signed_digest);
    }

    // Ensures that the a signature is valid for a given transaction
    pub fn verify_digest(&self) -> bool {
        // If the transaction is unsigned and isn't a coinbase transaction, it's invalid.
        if let None = self.signed_digest {
            if let Some(_) = self.sender_addr {
                return false;
            }
        }

        // Compute the digest
        let serialized: Vec<u8> = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        if let Ok(verified_data) = sign::verify(
            self.signed_digest.clone().unwrap().as_slice(),
            &self.sender_addr.unwrap(),
        ) {
            return digest == &verified_data[..];
        }

        false
    }

    // Creates a coinbase transactions to pay node that found nonce for a block
    pub fn create_coinbase_transaction(recipient_addr: PublicKey) -> Transaction {
        Transaction::new(None, recipient_addr, 1)
    }
}
