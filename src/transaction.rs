// Création d'une transaction normale de toute pièce en utilisant le wallet bitcoin Electrum

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