#![allow(unused)]
use bdk::blockchain::Blockchain;
use bdk::bitcoin::Address;
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