
use super::{Tulips, PublicKey, PrivateKey};
use sodiumoxide::crypto::{sign, hash};
use bincode::serialize;


#[derive(Serialize)]
pub struct Transaction {
    pub sender_addr: PublicKey,
    pub recipient_addr: PublicKey,
    pub value: Tulips,

    #[serde(skip)]
    pub signing_key: PrivateKey,
}

impl Transaction {
    pub fn sign(&mut self) -> Vec<u8> {
        // Serialize and hash the transaction
        let serialized: Vec<u8> = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        // And then sign the digest it with the senders private key
        sign::sign(digest, &self.signing_key);
    }

}
