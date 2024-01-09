use bitcoin::network::Network;
use bitcoin::PrivateKey;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;


fn private_key_to_wif(private_key: &str, network: Network) -> String {
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let private_key = PrivateKey::from_str(private_key).unwrap();
    let public_key = private_key.public_key(&secp);

    let extended_key = private_key.to_wif();

    extended_key
}

fn main() {
    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env");
    let network = Network::Testnet;

    let wif = private_key_to_wif(&private_key, network);
    println!("WIF: {}", wif);
}