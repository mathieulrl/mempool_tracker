use bitcoin::blockdata::opcodes;
use bitcoin::blockdata::script::{Builder, Script};
use bitcoin::blockdata::transaction::{TxIn, TxOut};
use bitcoin::consensus::encode::Decodable;
use bitcoin::hashes::hex::ToHex;
use electrum_client::{Client, Error};
use std::env;
use std::fmt;
use bitcoin::PrivateKey;
use bitcoin::Amount;
use serde::ser::StdError;

fn main() -> Result<(), Box<dyn StdError>> {
    let mut electrum = Client::new("127.0.0.1:50001").await?;

    // Get the list of unspent outputs
    let unspent_outputs = electrum.get_unspent_outputs().await?;

    // Select the first unspent output
    let utxo = unspent_outputs.first().unwrap();

    // Create a new transaction
    let mut tx = TxIn::new(utxo.outpoint_txid(), utxo.outpoint_vout());

    // Add the output
    let output = TxOut::new(Amount::from_sat(10000), Script::new(b""));
    tx.add_output(output);

    // Sign the transaction
    let private_key = PrivateKey::from_wif("your_private_key").unwrap();
    let signature = private_key.sign(&tx.get_sighash_all());
    tx.add_signature(signature);

    // Broadcast the transaction
    electrum.broadcast_transaction(&tx).await?;

    Ok(())
}