#![allow(unused)]
use bdk::{
    bitcoin::{self, Address, Network, Transaction},
    electrum_client::Client,
    wallet::{AddressIndex, SyncOptions, Wallet},
    blockchain::ElectrumBlockchain,
    database::MemoryDatabase,
};
use bdk::bitcoin::Txid;
use std::str::FromStr;
use bdk::blockchain::Blockchain;
use bdk::SignOptions;
use bdk::template::P2Wpkh;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);

    let key =
    bitcoin::PrivateKey::from_wif("cN89wfqNvFB8vRCMgmtKXwz24AyDxrtp6PDBLBKW1CqXWmxwMoe8")?;
    let wallet = Wallet::new(
        P2Wpkh(key),
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )?;
    

    // Synchroniser le portefeuille avec la blockchain
    wallet.sync(&blockchain, SyncOptions::default())?;

    // Remplacer le txid ci-dessous par celui de la transaction à surenchérir
    let existing_txid = bitcoin::Txid::from_str("d7363317230d4e2e93954ab8d1d570ba2e8145d392cce6cf6b88f9f6255818f3")?;
    let existing_transaction = wallet.get_tx(&existing_txid, true)?.expect("Transaction not found");
    



    // Calculer le nouveau montant de fee (par exemple, 2000 satoshis de plus que l'original)
    let new_fee_amount = match existing_transaction.fee {
        Some(value) => value + 2000,
        None => {
            // Si la transaction n'a pas de frais, nous devons en ajouter
            2000
        }
    };


    // Créer une nouvelle transaction en surenchérissant l'originale
    let mut tx_builder = wallet.build_tx();
    tx_builder
        .drain_wallet()  // Ajoute toutes les UTXO du portefeuille
        .add_recipient(
            Address::from_str("tb1qdqxq4lfq0kqdrt8vzcmtkse9lv9edd85c4q3qj")?.payload.script_pubkey(),
            new_fee_amount,
        )
        .enable_rbf();

    let (mut psbt, tx_details) = tx_builder.finish()?;

    // Signer la nouvelle transaction
    wallet.sign(&mut psbt, SignOptions::default())?;

    // Broadcast de la nouvelle transaction
    let raw_transaction = psbt.extract_tx();
    let txid = raw_transaction.txid();
    blockchain.broadcast(&raw_transaction)?;

    println!(
        "Transaction sent! TXID: {txid}.\nExplorer URL: https://blockstream.info/testnet/tx/{txid}",
        txid = txid
    );

    Ok(())
}
