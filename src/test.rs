use sodiumoxide::crypto::sign;
use super::{
    address,
    transaction,
    blockchain,
};

// After signing a transaction with a users private key,
// we should be able to verify that user signed it. If the user didn't, the verification should fail.
#[test]
fn test_signing() {
    // Create a pair of address
    let mut src_address = address::Address::new();
    let mut dest_address = address::Address::new();

    // Create a new transaction from source to destination
    // Because the sending account has 0 balance, the sending value is 0 tulips
    let (transaction, digest) = src_address.new_transaction(0, dest_address.public_key);

    // Verify that the transaction signature was signed by the private key of the sender
    assert!(transaction.verify_digest(digest));

    // Now, check that dest_address's attempt at tulip theft(!) is detected
    let forged_transaction = transaction::Transaction {
        sender_addr: Some(src_address.public_key),
        recipient_addr: dest_address.public_key,
        value: 100,
    };

    // The desination generates a private key and uses it to sign the transaction
    let (_public_key, private_key) = sign::gen_keypair();
    let forged_digest = forged_transaction.sign(&private_key);

    // Tulip theft averted!
    assert!(!transaction.verify_digest(forged_digest));
}

#[test]
fn test_proof_of_work() {
    // Create a blockchain
    let mut blockchain = blockchain::Blockchain::new();

    // Check that the chain with only the genesis block is valid
    assert!(blockchain.is_valid_chain());

    let nonce = blockchain.chain[0].nonce;
    blockchain.chain[0].nonce = 10;

    // Check that replacing the nonce invalidates the chain
    assert!(!blockchain.is_valid_chain());

    // Correc the nonce and ensure the chain is valid again
    blockchain.chain[0].nonce = nonce;
    assert!(blockchain.is_valid_chain());

    // Create a new zero-value transaction between two addresses and add it to the chain
    let mut src_address = address::Address::new();
    let mut dest_address = address::Address::new();
    let (transaction, digest) = src_address.new_transaction(0, dest_address.public_key);

    // Ensure the transaction is added to the list of pending transactions successfully
    assert!(blockchain.append_transaction(transaction, digest));

}
