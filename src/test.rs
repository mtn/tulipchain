use sodiumoxide::crypto::sign;
use super::{
    address,
    transaction,
    blockchain,
};

// After signing a transaction with a users private key, we should be able to verify
// that user signed it. If the user didn't, the verification should fail.
#[test]
fn test_signing() {
    // Create a pair of address
    let src_address = address::Address::new();
    let dest_address = address::Address::new();

    // Create a new transaction from source to destination
    // Because the sending account has 0 balance, the sending value is 0 tulips
    let transaction = src_address.new_transaction(0, dest_address.public_key);

    // Verify that the transaction signature was signed by the private key of the sender
    assert!(transaction.verify_digest());

    // Now, check that dest_address's attempt at tulip theft(!) is detected
    let mut forged_transaction = transaction::Transaction {
        sender_addr: Some(src_address.public_key),
        recipient_addr: dest_address.public_key,
        value: 100,
        signed_digest: None,
    };

    // The desination generates a private key and uses it to sign the transaction
    let (_public_key, private_key) = sign::gen_keypair();
    forged_transaction.sign(&private_key);

    // Tulip theft averted!
    assert!(!forged_transaction.verify_digest());
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
    let src_address = address::Address::new();
    let dest_address = address::Address::new();
    let transaction = src_address.new_transaction(0, dest_address.public_key);

    // Ensure the transaction is added to the list of pending transactions successfully
    let transaction_clone = transaction.clone();
    assert!(blockchain.append_transaction(transaction));
    assert!(transaction_clone == blockchain.pending_transactions[0]);

    let last_nonce = blockchain.get_last_nonce();
    // Find a nonce to add the block to the chain
    let nonce = blockchain::Blockchain::find_nonce(last_nonce, &None);

    let payout_addr = blockchain.address.clone().unwrap().public_key;
    // Add a new block to the chain using the nonce that was found
    blockchain.append_block(nonce, None, payout_addr);

    // Ensure that the block got added and that it contains the transaction
    assert!(blockchain.chain.len() == 2);
    // With a mining abstraction, there will be an additional coinbase transaction
    assert!(blockchain.chain.last().unwrap().transactions.len() == 1);

    println!("{:?}", blockchain.chain.last().unwrap());
}
