#[macro_use]
extern crate serde_derive;
extern crate sodiumoxide;
extern crate bincode;
extern crate chrono;
extern crate rand;

mod transaction;
mod blockchain;
mod address;

type SignedDigest = Vec<u8>;
type Digest = Vec<u8>;
type Tulips = u32;

use sodiumoxide::crypto::sign;
type PublicKey = sign::ed25519::PublicKey;
type PrivateKey = sign::ed25519::SecretKey;

fn main() {
    let (public, private) = sign::gen_keypair();
    let mut transaction = transaction::Transaction {
        sender_addr: public,
        recipient_addr: public,
        value: 10,
    };


    // let encoded: Vec<u8> = serialize(&transaction).unwrap();
    // println!("{:?}", encoded);
    // println!("{:?}", encoded.len());
    let signed = transaction.sign(private);
    let res = transaction.verify_digest(signed);
    println!("res {}", res);
    // println!("{:?}", pk);
    // let data_to_sign = b"some data";
    // let signed_data = sign::sign(data_to_sign, &sk);
    // let verified_data: Vec<u8> = sign::verify(&signed_data, &pk).unwrap();
    // assert!(data_to_sign == &verified_data[..]);
    // println!("{:?}", sign::sign(data_to_sign, &sk));

    println!("Hello, world!");
}
