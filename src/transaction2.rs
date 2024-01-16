#![allow(unused)]
use bdk::blockchain::Blockchain;
use bdk::bitcoin::Address;
use serde_json::from_str;
use std::str::FromStr;
use bdk::SignOptions;
use bdk::SyncOptions;
use bdk::{
    blockchain::ElectrumBlockchain,
    electrum_client::Client,
    descriptor,
};
use bdk::template::P2Wpkh;
use bdk::wallet::AddressIndex::New;
use bdk::{Wallet, bitcoin, bitcoin::Network, database::MemoryDatabase};
use bdk::wallet::AddressIndex;

use bitcoin::consensus::serialize;
use miniscript::bitcoin::util::psbt;
use bitcoin::Script;
use bitcoin::opcodes::OP_TRUE;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let key =
    bitcoin::PrivateKey::from_wif("")?;
    let wallet = Wallet::new(
        P2Wpkh(key),
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )?;
    
//     let external_key =
//     bitcoin::PrivateKey::from_wif("cVFnr9ZQzXVrEKRBoEzJ97HfzjrKJCStLN2Q3eAquY33ndpsbCZ1")?;
//     let (external_descriptor, key_map_1, networks_1) = bdk::descriptor!(wpkh(external_key))?;

//     let internal_key =
//     bitcoin::PrivateKey::from_wif("cVaAjQfVNjRVUp9CPZfYhSMhJKyaCfEzZ9esx8NtrFBt3vgWKuHD")?;
//     let (internal_descriptor, key_map_2, networks_2) = bdk::descriptor!(wpkh(internal_key))?;


// //    let external_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
// //    let internal_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/1/*)";

//     let wallet: Wallet<MemoryDatabase> = Wallet::new(
//         external_descriptor,
//         Some(internal_descriptor),
//         Network::Testnet,
//         MemoryDatabase::new(),
//     )?;
//     // ...   

    

    let address = wallet.get_address(AddressIndex::New)?;
    println!("Generated Address: {}", address);

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);

    wallet.sync(&blockchain, SyncOptions::default())?;

    let balance = wallet.get_balance()?;
    println!("Wallet balance in SAT: {}", balance);
    let faucet_address = Address::from_str("tb1qlwjhp2nlf8tuk9n574rm0f7q2qjwpfa0jdpfu4")?;

    // create a transaction
    let mut tx_builder = wallet.build_tx();
    tx_builder
        .add_recipient(
            faucet_address.payload.script_pubkey(),
            1000,
        )
        .enable_rbf();
    let (mut psbt, tx_details) = tx_builder.finish()?;

    println!("Transaction details: {:#?}", tx_details);

    // Modify the 2 first hex digits of the script pubkey of the output
    let output_index = 1; // Index of the output you want to modify
    let mut script_pubkey = psbt.unsigned_tx.output[output_index].script_pubkey.clone();
    let mut script_pubkey_bytes = script_pubkey.into_bytes();

    // Modify the first two hex digits
    script_pubkey_bytes[0] = 0x51;


    // Set the new script pubkey
    let new_script_pubkey = Script::from_bytes(&script_pubkey_bytes);
    psbt.unsigned_tx.output[output_index].script_pubkey = new_script_pubkey.into();

//     // Modify the script pubkey of the output to use OP_TRUE (anyone can spend)
//     let output_index = 0; // Index of the output you want to modify

//     let mut builder = bitcoin::blockdata::script::Builder::new();
//     builder = builder.push_opcode(bitcoin::blockdata::opcodes::OP_TRUE);

//     // Ajouter la clé publique du destinataire
//    let receiver_pubkey = bitcoin::PublicKey::from_str("tb1qlwjhp2nlf8tuk9n574rm0f7q2qjwpfa0jdpfu4");
    
//    match receiver_pubkey {
//     Ok(receiver_pubkey) => {
//         builder = builder.push_key(&receiver_pubkey);
//     }
//     Err(e) => {
//         println!("Error parsing public key: {}", e);
//     }
// }

//     // Définir le nouveau script pubkey
//     let output = &mut psbt.unsigned_tx.output[output_index];
//     output.script_pubkey = builder.into_script();


    let finalized = wallet.sign(&mut psbt,SignOptions::default())?;
    assert!(finalized,"Tx has not been finalized");
    println!("Transaction Signed: {}", finalized);


    // broadcast the transaction
    let raw_transaction = psbt.extract_tx();
    let txid = raw_transaction.txid();
    blockchain.broadcast(&raw_transaction)?;
    println!(
    "Transaction sent! TXID: {txid}.\nExplorer URL: https://blockstream.info/testnet/tx/{txid}",
    txid = txid
);

    Ok(())
}