use sodiumoxide::crypto::sign;
use super::{
    address,
    transaction,
};

// After signing a transaction with a users private key,
// we should be able to verify that user signed it. If the user didn't, the verification should fail.
#[test]
fn test_signing() {
    // Create an address
    let mut src_address = address::Address::new();
    let mut dest_address = address::Address::new();

    // Create a new transaction from source to destination
    // Because the sending account has 0 balance, the sending value is 0 tulips
    let (transaction, digest) = src_address.new_transaction(0, dest_address.public_key);

    // Verify that the transaction signature was signed by the private key of the sender
    assert!(transaction.verify_digest(digest));

    // Now, check that dest_address's attempt at tulip theft(!) is detected
    let forged_transaction = transaction::Transaction {
        sender_addr: src_address.public_key,
        recipient_addr: dest_address.public_key,
        value: 100,
    };

    // The desination generates a private key and uses it to sign the transaction
    let (_public_key, private_key) = sign::gen_keypair();
    let forged_digest = forged_transaction.sign(&private_key);

    // Tulip theft averted!
    assert!(!transaction.verify_digest(forged_digest));
}
