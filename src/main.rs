#[macro_use]
extern crate serde_derive;
extern crate sodiumoxide;
extern crate bincode;
extern crate chrono;
extern crate rand;

mod transaction;
mod blockchain;
mod address;

#[cfg(test)]
mod test;

type SignedDigest = Vec<u8>;
type Digest = Vec<u8>;
type Tulips = u32;

use sodiumoxide::crypto::sign;
type PublicKey = sign::ed25519::PublicKey;
type PrivateKey = sign::ed25519::SecretKey;

fn main() {
    println!("Hello, world!");
}
