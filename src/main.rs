// Code for retrieving information about one or multiple transactions on mempool.space.

use reqwest::header;
use hex;
use tokio::time::interval;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));


struct InterestingTx {
    txid: String,
    fees: u64,
}

// Function to create and return a reqwest Client with default headers
fn get_client() -> reqwest::Client {
    let headers = header::HeaderMap::new();

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    client
}


#[tokio::main]
async fn main() {
    // Vector to store interesting transactions
    let mut tab_interesting_tx = Vec::new();

    // URL to retrieve transaction IDs from mempool.space
    let txids_url = "https://mempool.space/testnet/api/mempool/txids";

    // Create reqwest client
    let client = get_client();

    // Send GET request to obtain transaction IDs
    let txids_response = client.get(txids_url).send().await.unwrap();

    // Check if the request was successful
    if txids_response.status().is_success() {
        // Parse JSON response into vector of transaction IDs
        let txids = txids_response.json::<Vec<String>>().await.unwrap();

        // Iterate over each transaction ID
        for txid in &txids {
            // URL to retrieve detailed transaction information
            let tx_url = format!("https://mempool.space/testnet/api/tx/{}", &txid);

            // Send GET request to obtain transaction details
            let tx_response = client.get(&tx_url).send().await.unwrap();

            // Check if the request was successful
            if tx_response.status().is_success() {
                // Parse JSON response into serde_json::Value
                let tx_json = tx_response.json::<serde_json::Value>().await.unwrap();

                // Extract input and output information from transaction details
                let vin = tx_json["vin"].as_array().unwrap();
                let vout = tx_json["vout"].as_array().unwrap(); 

                // Extract original transaction fee
                let fees_origin = tx_json["fee"].as_u64().unwrap();

                // Iterate over each input in the transaction
                for input in vin {
                    // Extract scriptSig and witness information
                    let scriptsig = input["scriptsig"].as_str().unwrap_or("");
                    let witness = input["witness"].as_array().map(|arr| arr.to_vec()).unwrap_or_else(Vec::new);

                    // Process witness information
                    let first_witness = &witness.first();
                    let mut decoded_witness = Vec::new();

                    for witness_value in first_witness {
                        if let Some(witness_str) = witness_value.as_str() {
                            decoded_witness = hex::decode(witness_str).expect("Failed to decode witness");
                        }
                    }

                    // Check if the last value in the witness is not equal to 1
                    if let Some(last_value) = decoded_witness.last() {
                        if last_value != &1 {
                            println!("La dernière valeur du premier vecteur de witness est différente de 1 : {}", last_value);
                            // Add interesting transaction to the vector
                            tab_interesting_tx.push(InterestingTx {
                                txid: txid.to_string(),
                                fees: fees_origin,
                            });
                            println!("txid: {}", txid);
                            println!("fees: {}", fees_origin);
                        }
                    }
                }

                // Iterate over each output in the transaction
                for output in vout {
                    // Extract scriptPubKey information
                    let script_pub_key = output["scriptpubkey"].as_str().unwrap_or("");
                    // println!("scriptPubKey: {}", &script_pub_key);
                    
                    // Check if scriptPubKey starts with "51"
                    if script_pub_key.starts_with("51") {
                        println!("ScriptPubKey starts with '51'");
                        // Add interesting transaction to the vector
                        tab_interesting_tx.push(InterestingTx {
                            txid: txid.to_string(),
                            fees: fees_origin,
                        });
                        println!("txid: {}", txid);
                        println!("fees: {}", fees_origin);
                    }
                }
            } else {
                // Print an error message if the request fails
                println!("Request failed with status code: {}", tx_response.status());
            }
        }
    } else {
        // Print an error message if the request for transaction IDs fails
        println!("Request failed with status code: {}", txids_response.status());
    }
}
