
use sodiumoxide::crypto::{sign, hash};
use bincode::serialize;
use super::{
    Tulips,
    PublicKey,
    PrivateKey,
    SignedDigest,
};


#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Transaction {
    // Coinbase transactions have no sender
    pub sender_addr: Option<PublicKey>,
    pub recipient_addr: PublicKey,
    pub value: Tulips,

    #[serde(skip)]
    pub signed_digest: Option<SignedDigest>,
}

impl Transaction {
    pub fn sign(&mut self, signing_key: &PrivateKey) {
        let serialized = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        // Sign the digest with the senders private key
        let signed_digest = sign::sign(digest, signing_key);
        self.signed_digest = Some(signed_digest);
    }

    // Ensures that the a signature is valid for a given transaction
    pub fn verify_digest(&self)
        -> bool {

        // If the transaction is unsigned and isn't a coinbase transaction, it's invalid.
        if let None = self.signed_digest {
            if let Some(_) = self.sender_addr {
                return false
            }
        }

        // Compute the digest
        let serialized: Vec<u8> = serialize(self).unwrap();
        let hash::sha256::Digest(ref digest) = hash::sha256::hash(&serialized);

        if let Ok(verified_data) = sign::verify(self.signed_digest.clone().unwrap().as_slice(),
                                                &self.sender_addr.unwrap()) {
            return digest == &verified_data[..]
        }

        false
    }

}
