use sodiumoxide::crypto::sign;
use super::{Tulips, PublicKey, PrivateKey};

pub struct Address {
    pub public_key: PublicKey,
    pub private_key: PrivateKey,

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
}
