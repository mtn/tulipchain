
use sodiumoxide::crypto::{sign, hash};
use bincode::serialize;
use super::{
    Tulips,
    PublicKey,
    PrivateKey,
    SignedDigest,
};


#[derive(Debug, Serialize, Clone)]
pub struct Transaction {
    // Coinbase transactions have no sender
    pub sender_addr: Option<PublicKey>,
    pub recipient_addr: PublicKey,
    pub value: Tulips,
}

impl Transaction {
    pub fn sign(&self, signing_key: &PrivateKey) -> SignedDigest {
        let serialized = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        // Sign the digest with the senders private key
        sign::sign(digest, signing_key)
    }

    // Ensures that the a signature is valid for a given transaction
    pub fn verify_digest(&self, signed_digest: SignedDigest)
        -> bool {

        // Compute the digest
        let serialized: Vec<u8> = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        if let Ok(verified_data) = sign::verify(&signed_digest,
                                                &self.sender_addr.unwrap()) {
            return digest == &verified_data[..]
        }

        false
    }

}
