use electrum_client::ElectrumApi;
use electrum_client::raw_client::RawClient;
use electrum_client::{ElectrumApiError, ElectrumApiResult};
use bitcoin::network::constants::Network;
use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut};
use bitcoin::blockdata::opcodes;
use bitcoin::blockdata::script::{Builder, Script};
use bitcoin::util::address::Address;
use bitcoin::util::amount::Amount;
use bitcoin::util::key::PrivateKey;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::hashes::hex::ToHex;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    dotenv().ok();

    let mut electrum = RawClient::new("127.0.0.1:50001", Network::Bitcoin).await?;

    // Remplacez ces valeurs par vos propres données
    let private_key_wif = "private_key";
    let destination_address = "tb1qdqxq4lfq0kqdrt8vzcmtkse9lv9edd85c4q3qj";
    let amount_satoshi = 10000;

    // Convertir la clé privée WIF en clé privée Bitcoin
    let private_key = PrivateKey::from_wif(private_key_wif)?;

    // Obtenir l'adresse de sortie
    let output_address = Address::from_str(tb1qdqxq4lfq0kqdrt8vzcmtkse9lv9edd85c4q3qj)?;

    // Obtenir des informations sur l'UTXO à dépenser
    let unspent_outputs = electrum.list_unspent().await?;

    // Créer l'entrée de transaction (UTXO)
    let utxo = select_utxo(unspent_outputs)?;
    let txin = create_txin(&utxo);

    // Créer la sortie de transaction
    let txout = create_txout(&output_address, amount_satoshi)?;

    // Créer la transaction avec SIGHASH_NONE|ANYONECANPAY
    let tx = create_transaction(vec![txin], vec![txout], private_key)?;

    // Afficher la transaction hexadécimale
    println!("Transaction hex: {}", tx.txid().to_hex());

    // Envoyer la transaction à Electrum pour diffusion
    electrum.send_raw_transaction(&tx).await?;

    Ok(())
}

fn select_utxo(unspent_outputs: Vec<electrum_client::UnspentOutput>) -> Result<electrum_client::UnspentOutput, Box<dyn std::error::Error>> {
    // Dans cet exemple, nous sélectionnons simplement la première UTXO disponible.
    // Vous pouvez implémenter votre propre logique de sélection ici.
    match unspent_outputs.into_iter().next() {
        Some(utxo) => Ok(utxo),
        None => Err(Box::new(ElectrumApiError::Other(String::from("No UTXO available")))),
    }
}

fn create_txin(utxo: &electrum_client::UnspentOutput) -> TxIn {
    let outpoint = utxo.outpoint_txid();
    let outpoint_index = utxo.outpoint_vout();
    let sequence = 0x000000ff; // SIGHASH_NONE|ANYONECANPAY

    TxIn::new(outpoint, sequence)
}

fn create_txout(address: &Address, amount_satoshi: u64) -> Result<TxOut, Box<dyn std::error::Error>> {
    let script = address.script_pubkey();
    let amount = Amount::from_sat(amount_satoshi);

    Ok(TxOut {
        script_pubkey: script,
        value: amount,
    })
}

fn create_transaction(txins: Vec<TxIn>, txouts: Vec<TxOut>, private_key: PrivateKey) -> Result<Transaction, Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    let mut tx = Transaction {
        version: 1,
        lock_time: 0,
        input: txins,
        output: txouts,
    };

    // Signer les entrées de transaction
    for i in 0..tx.input.len() {
        let input = &mut tx.input[i];

        let script_code = Builder::new()
            .push_opcode(opcodes::all::OP_TRUE)
            .into_script();

        let sighash = tx.signature_hash(i, &script_code, 0x000000ff); // SIGHASH_NONE|ANYONECANPAY

        let signature = secp.sign(&sighash, &private_key.key)?;
        let mut signature = signature.serialize_der().to_vec();
        signature.push(0x01); //Notez que l'exemple de code ci-dessus utilise la bibliothèque `electrum-client` pour interagir avec Electrum et la bibliothèque `bitcoin` pour manipuler les transactions Bitcoin. Assurez-vous d'ajouter ces dépendances à votre fichier `Cargo.toml` :
    }

