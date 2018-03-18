
use sodiumoxide::crypto::{sign, hash};
use bincode::serialize;
use super::{
    Tulips,
    PublicKey,
    PrivateKey,
    SignedDigest,
};


#[derive(Serialize)]
pub struct Transaction {
    pub sender_addr: PublicKey,
    pub recipient_addr: PublicKey,
    pub value: Tulips,
}

impl Transaction {
    pub fn sign(& mut self, signing_key: PrivateKey) -> SignedDigest {
        let serialized = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        // Sign the digest with the senders private key
        sign::sign(digest, &signing_key)
    }

    // Ensures that the a signature is valid for a given transaction
    pub fn verify_digest(&self, signed_digest: SignedDigest)
        -> bool {

        // Compute the digest
        let serialized: Vec<u8> = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        let verified_data = sign::verify(&signed_digest, &self.sender_addr).unwrap();

        digest == &verified_data[..]
    }
}
