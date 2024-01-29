// The purpose of this code is to convert the P2WPKH format provided by Electrum (when requesting the private key) into the WPKH format required in the transaction2.rs code.

use bitcoin::secp256k1::PublicKey;
use bitcoin::util::bip32::{ChildNumber, ExtendedPubKey};
use miniscript::descriptor::{Descriptor, DescriptorPublicKey, DescriptorSinglePub};
use miniscript::policy::Concrete;
use miniscript::ScriptContext;
use bdk::keys::ScriptContext;

fn main() {
    // P2WPKH script
    let p2wpkh_script = hex::decode("0014{public_key_hash}").unwrap();

    // Public key hash
    let public_key_hash = hex::decode("{public_key_hash}").unwrap();

    // Create a WPKH descriptor
    let descriptor = Descriptor::<PublicKey>::Wpkh(
        DescriptorSinglePub::new(
            PublicKey::from_slice(&public_key_hash).unwrap(),
            None,
        )
        .unwrap(),
    );

    let descriptor_string = descriptor.to_string();

    println!("WPKH Descriptor: {}", descriptor_string);
}


use bitcoin::secp256k1::PublicKey;
use bitcoin::util::bip32::{ChildNumber, ExtendedPubKey};
use miniscript::descriptor::{Descriptor, DescriptorPublicKey, DescriptorSinglePub};
use miniscript::ScriptContext;

fn main() {
    
    let public_key_hash = hex::decode("PUBLIC_KEY_HASH").unwrap();

    // Creation of script P2WPKH
    let p2wpkh_script = hex::decode(format!("0014{:x}", public_key_hash.len()))
        .unwrap()
        .into_iter()
        .chain(public_key_hash.into_iter())
        .collect::<Vec<_>>();

    // Creation of descriptor WPKH
    let descriptor = Descriptor::<PublicKey>::Wpkh(
        DescriptorSinglePub::new(
            PublicKey::from_slice(&public_key_hash).unwrap(),
            None,
        )
        .unwrap(),
    );

    //Printing the descriptor
    let descriptor_string = format!("{}", descriptor);
    println!("WPKH Descriptor: {}", descriptor_string);
}