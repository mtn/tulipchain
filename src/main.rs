#[macro_use]
extern crate serde_derive;

extern crate bincode;
use bincode::serialize;

extern crate sodiumoxide;
use sodiumoxide::crypto::sign;

mod transaction;
mod blockchain;
mod address;

type Tulips = u32;
type PublicKey = sign::ed25519::PublicKey;
type PrivateKey = sign::ed25519::SecretKey;


fn main() {
    let (pk, sk) = sign::gen_keypair();
    let mut transaction = transaction::Transaction {
        sender_addr: pk,
        recipient_addr: pk,
        value: 10,
        signing_key: sk
    };

    // let encoded: Vec<u8> = serialize(&transaction).unwrap();
    // println!("{:?}", encoded);
    // println!("{:?}", encoded.len());
    //
    transaction.sign();




    // println!("{:?}", pk);
    // let data_to_sign = b"some data";
    // let signed_data = sign::sign(data_to_sign, &sk);
    // let verified_data: Vec<u8> = sign::verify(&signed_data, &pk).unwrap();
    // assert!(data_to_sign == &verified_data[..]);
    // println!("{:?}", sign::sign(data_to_sign, &sk));

    println!("Hello, world!");
}
