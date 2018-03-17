
use super::{Tulips, PublicKey, PrivateKey};

pub struct Address {
    pub public_key: PublicKey,
    pub private_key: PublicKey,

    pub balance: Tulips,
}
